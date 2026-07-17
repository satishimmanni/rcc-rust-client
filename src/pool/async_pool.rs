use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
    sync::{
        Arc,
        atomic::{AtomicU16, Ordering},
    },
};

use crate::{
    cluster::client::{self, Client},
    commands::{
        command::RetryReq,
        command_req::{CmdParser, ReqSerializer},
        command_res::CommandRes,
        connection::{ConnectReq, ConnectRes, ConnectionType},
    },
};

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::{
    io::{AsyncWriteExt, BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

struct TxReq {
    data: Box<[u8]>,
    resp_tx: oneshot::Sender<Result<CommandRes, Error>>,
}

pub struct AsyncPool {
    senders: Arc<Vec<mpsc::Sender<TxReq>>>,
    current_index: AtomicU16,
    size: u16,
    host: String,
    port: u16,
}

impl std::fmt::Debug for AsyncPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncPool")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("size", &self.size)
            .finish()
    }
}

impl AsyncPool {
    pub async fn connect(
        client: Arc<Client>,
        pool_size: u16,
        host: &str,
        port: u16,
        access_id: String,
        access_token: String,
    ) -> Self {
        let pool_size = pool_size.max(1);
        let mut senders = Vec::with_capacity(pool_size as usize);

        let mut pending_connections: FuturesUnordered<_> = (0..pool_size)
            .map(|_| {
                establish_connection(
                    client.clone(),
                    host,
                    port,
                    access_id.clone(),
                    access_token.clone(),
                )
            })
            .collect();

        while let Some(initial) = pending_connections.next().await {
            let initial = match initial {
                Ok(conn) => Some(conn),
                Err(e) => {
                    tracing::error!("failed to connect to {}:{}: {}", host, port, e);
                    None
                }
            };
            let (sender, receiver) = mpsc::channel::<TxReq>(1024);
            senders.push(sender);

            tokio::spawn(connection_task(
                client.clone(),
                receiver,
                initial,
                host.to_string(),
                port,
                access_id.clone(),
                access_token.clone(),
            ));
        }
        AsyncPool {
            senders: Arc::new(senders),
            current_index: AtomicU16::new(0),
            size: pool_size,
            host: host.to_string(),
            port,
        }
    }

    pub async fn send(&self, client: Arc<Client>, req: RetryReq) -> Result<CommandRes, Error> {
        let data = req.command.serialize(
            client.get_client_id(),
            client.get_hash(),
            client.get_last_updated_at(),
        );

        let idx = self.next_index();
        let (resp_tx, resp_rx) = oneshot::channel();

        self.senders[idx]
            .send(TxReq { data, resp_tx })
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "connection writer task ended"))?;

        resp_rx
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "connection closed"))?
    }

    fn next_index(&self) -> usize {
        loop {
            let cur = self.current_index.load(Ordering::Relaxed);
            let next = if cur + 1 >= self.size { 0 } else { cur + 1 };
            if self
                .current_index
                .compare_exchange_weak(cur, next, Ordering::Relaxed, Ordering::Relaxed)
                .is_ok()
            {
                return cur as usize;
            }
        }
    }
}

type ReaderMsg = Result<CommandRes, Error>;

struct Conn {
    writer: BufWriter<OwnedWriteHalf>,
    resp_rx: mpsc::UnboundedReceiver<ReaderMsg>,
    reader_handle: JoinHandle<()>,
}

async fn establish_connection(
    client: Arc<Client>,
    host: &str,
    port: u16,
    access_id: String,
    access_token: String,
) -> Result<Conn, Error> {
    let (reader, writer) = connect(client.clone(), host, port, access_id, access_token).await?;
    let (tx, rx) = mpsc::unbounded_channel();
    let reader_handle = tokio::spawn(reader_task(client, reader, tx));
    Ok(Conn {
        writer,
        resp_rx: rx,
        reader_handle,
    })
}

async fn connection_task(
    client: Arc<Client>,
    mut receiver: mpsc::Receiver<TxReq>,
    initial: Option<Conn>,
    host: String,
    port: u16,
    access_id: String,
    access_token: String,
) {
    let mut conn = initial;
    let mut pending: VecDeque<oneshot::Sender<Result<CommandRes, Error>>> = VecDeque::new();

    loop {
        tokio::select! {
            biased;

            maybe_res = async {
                match conn.as_mut() {
                    Some(c) => c.resp_rx.recv().await,
                    None => std::future::pending().await,
                }
            } => {
                match maybe_res {
                    Some(res) => match pending.pop_front() {
                        Some(sender) => {
                            let err_detail = res.as_ref().err().map(|e| (e.kind(), e.to_string()));
                            _ =sender.send(res);
                            if let Some((kind, msg)) = err_detail {
                                fail_connection(&mut conn, &mut pending, kind, &msg);
                            }
                        }
                        None => {
                            fail_connection(&mut conn, &mut pending, ErrorKind::Other, "connection closed");
                        }
                    },
                    None => {
                        fail_connection(&mut conn, &mut pending, ErrorKind::Other, "connection closed");
                    }
                }
            }
            maybe_req = receiver.recv() => {
                let Some(first) = maybe_req else { break };
                let mut reqs = vec![first];
                while let Ok(bulk_req) = receiver.try_recv() {
                    reqs.push(bulk_req);
                }

                if conn.is_none() {
                    match establish_connection(client.clone(),&host, port, access_id.clone(), access_token.clone()).await {
                        Ok(c) => conn = Some(c),
                        Err(err) => {
                            for req in reqs {
                                let _ = req.resp_tx.send(Err(Error::new(err.kind(), err.to_string())));
                            }
                            continue;
                        }
                    }
                }

                let mut buf = Vec::with_capacity(reqs.iter().map(|r| r.data.len()).sum());
               // tracing::debug!("reqs in one go: {}", reqs.len());
                for req in reqs {
                    pending.push_back(req.resp_tx);
                    buf.extend_from_slice(&req.data);
                }

                let c = conn.as_mut().unwrap();
                let mut write_err = c.writer.write_all(&buf).await.err();
                if write_err.is_none() {
                    if let Err(err) = c.writer.flush().await {
                        write_err = Some(err);
                    }
                }

                if let Some(err) = write_err {
                    fail_connection(&mut conn, &mut pending, err.kind(), &err.to_string());
                }
            }
        }
    }

    if let Some(c) = conn.take() {
        c.reader_handle.abort();
    }
}

fn fail_connection(
    conn: &mut Option<Conn>,
    pending: &mut VecDeque<oneshot::Sender<Result<CommandRes, Error>>>,
    kind: ErrorKind,
    msg: &str,
) {
    if let Some(c) = conn.as_mut() {
        while let Ok(res) = c.resp_rx.try_recv() {
            match pending.pop_front() {
                Some(sender) => {
                    _ = sender.send(res);
                }
                None => break,
            }
        }
    }
    if let Some(c) = conn.take() {
        c.reader_handle.abort();
    }
    for sender in pending.drain(..) {
        let _ = sender.send(Err(Error::new(kind, msg)));
    }
}

async fn reader_task(
    client: Arc<Client>,
    mut reader: BufReader<OwnedReadHalf>,
    resp_tx: mpsc::UnboundedSender<ReaderMsg>,
) {
    loop {
        let res = CommandRes::parse(&mut reader, client.clone()).await;
        let is_err = res.is_err();
        if resp_tx.send(res).is_err() || is_err {
            return;
        }
    }
}

async fn connect(
    client: Arc<Client>,
    host: &str,
    port: u16,
    access_id: String,
    access_token: String,
) -> Result<(BufReader<OwnedReadHalf>, BufWriter<OwnedWriteHalf>), Error> {
    let tcp_client = TcpStream::connect(format!("{}:{}", host, port)).await?;
    _ = tcp_client.set_nodelay(true);
    let (read, write) = tcp_client.into_split();

    let mut reader = BufReader::new(read);
    let mut writer = BufWriter::new(write);
    let connect_req = ConnectReq {
        connection_type: ConnectionType::Client,
        access_id: access_id,
        access_token,
    };

    writer.write_all(&connect_req.serialize()).await?;
    writer.flush().await?;
    tracing::debug!("sent connect req,{}:{}", host, port);
    let opt_res = ConnectRes::parse(&mut reader).await;
    tracing::debug!("connect res: {:?}", opt_res);
    match opt_res {
        Ok((_, opt_cluster)) => {
            if let Some(cluster) = opt_cluster {
                if client.get_hash() != cluster.hash
                    && client.get_last_updated_at() < cluster.last_updated_at
                {
                    tracing::debug!("update cluster from connect res parser");
                    client::update_cluster(client, cluster).await;
                }
            }
            Ok((reader, writer))
        }
        Err(_) => Err(Error::new(ErrorKind::Other, "Server closed connection")),
    }
}
