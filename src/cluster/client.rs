use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicU64, Ordering},
    },
};

use arc_swap::{ArcSwap, ArcSwapOption};
use chrono::Utc;
use dashmap::DashMap;
use futures::{FutureExt, future::BoxFuture};

use rand::seq::IndexedRandom;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use xxhash_rust::xxh3::xxh3_64;

use crate::{
    cluster::{
        ServerAddress,
        cluster_state::ClusterState,
        node::Node,
        token_range_replica::{TokenRangeReplicas, TokenRangeReplicasDTO},
    },
    commands::{command::RetryReq, command_req::Req, command_res::CommandRes},
    pool::async_pool::AsyncPool,
};

#[derive(Debug)]
pub struct Client {
    pub id: Uuid,
    pub hash: AtomicU64,
    pub last_updated_at: AtomicI64,
    pub is_updating: AtomicBool,
    pub cons_count: u16,
    pub cluster: ArcSwapOption<Cluster>,
    pub access_id: String,
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClusterDTO {
    pub id: Uuid,
    pub cluster_state: ClusterState,
    pub hash: u64,
    pub last_updated_at: i64,
    //pub v_node_ring: VNodeRingDTO,
    pub token_range_replicas: Vec<TokenRangeReplicasDTO>,
    pub node_id: u64,
}

#[derive(Debug)]
pub struct Cluster {
    pub id: Uuid,
    pub cluster_state: AtomicU8,
    pub token_range_replicas: ArcSwap<Vec<TokenRangeReplicas>>,
    pub nodes: ArcSwap<DashMap<u64, Arc<Node>>>,
}

// static CLUSTER: OnceLock<Cluster> = OnceLock::new();
// static CLIENT: OnceLock<Client> = OnceLock::new();

impl Client {
    pub async fn boot(
        initial_server: ServerAddress,
        cons_count: u16,
        access_id: String,
        access_token: String,
    ) -> Result<Arc<Self>, Error> {
        tracing::debug!("boot");
        let client = Arc::new(Client {
            id: Uuid::new_v4(),
            hash: AtomicU64::new(0),
            last_updated_at: AtomicI64::new(0),
            is_updating: AtomicBool::new(false),
            cons_count: cons_count,
            cluster: ArcSwapOption::empty(),
            access_id,
            access_token,
        });

        let _pool = AsyncPool::connect(
            client.clone(),
            1,
            &initial_server.host,
            initial_server.port,
            client.access_id.clone(),
            client.access_token.clone(),
        )
        .await;
        tracing::debug!("boot pool connected");

        Ok(client.clone())
    }

    pub fn get_key_hash(key: &[u8]) -> u64 {
        xxh3_64(key) % 65536
    }

    pub fn get_client_id(&self) -> Uuid {
        self.id
    }

    pub fn get_cluster_state(&self) -> Result<ClusterState, Error> {
        if let Some(cluster) = self.cluster.load_full() {
            let cluster_state = cluster.cluster_state.load(Ordering::Relaxed);
            return ClusterState::try_from(cluster_state);
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            "Cluster not initiatilized",
        ))
    }

    pub fn set_cluster_state(&self, cluster_state: ClusterState) {
        if let Some(cluster) = self.cluster.load_full() {
            cluster
                .cluster_state
                .store(cluster_state.into(), Ordering::Relaxed);
        }
    }

    pub fn set_hash(&self, hash: u64) {
        self.hash.store(hash, Ordering::Relaxed);
    }

    pub fn get_cons_size(&self) -> u16 {
        self.cons_count
    }

    pub fn set_last_updated_at(&self, last_updated_at: i64) {
        self.last_updated_at
            .store(last_updated_at, Ordering::Relaxed);
    }

    pub fn get_hash(&self) -> u64 {
        self.hash.load(Ordering::Relaxed)
    }

    pub fn get_last_updated_at(&self) -> i64 {
        self.last_updated_at.load(Ordering::Relaxed)
    }
    pub fn is_updating(&self) -> bool {
        self.is_updating.load(Ordering::SeqCst)
    }
    pub fn set_updating(&self, status: bool) {
        self.is_updating.store(status, Ordering::SeqCst)
    }
}

pub async fn send(client: Arc<Client>, req: RetryReq) -> Result<CommandRes, Error> {
    let cluster = match client.cluster.load_full() {
        Some(cluster) => cluster,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Cluster not initiatilized to send req",
            ));
        }
    };

    let trs = cluster.token_range_replicas.load_full();
    if trs.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Cluster has no token ranges yet",
        ));
    }

    let pool =
        match &req.command {
            Req::Command(key_req) => {
                let hash = Client::get_key_hash(key_req.key.as_bytes());
                let n = match trs.binary_search_by(|tr| tr.token_range.end.cmp(&hash)) {
                    Ok(n) => n,
                    Err(n) => n % trs.len(),
                };
                let tr = &trs[n];
                let replica = tr.replicas.first().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Token range has no replicas")
                })?;
                replica.pool.load_full()
            }
            Req::Cfg(_cfg_req) => {
                let mut rng = rand::rng();
                // trs was checked non-empty above, so this always yields Some.
                let tr = trs.choose(&mut rng).expect("trs is non-empty");
                let node = tr.replicas.choose(&mut rng).ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "Token range has no replicas")
                })?;
                node.pool.load_full()
            }
        };
    pool.send(client, req).await
}

pub async fn init(client: Arc<Client>, cluster_dto: ClusterDTO) {
    // tracing::debug!("initiating cluster: {:?}", cluster_dto);
    let trr = get_token_range_replicas(
        client.clone(),
        cluster_dto.token_range_replicas,
        HashMap::new(),
    )
    .await;
    let nodes = DashMap::new();
    for trr in trr.iter() {
        if let Some(replica) = trr.replicas.first() {
            nodes.insert(replica.id, replica.clone());
        }
    }
    let cluster = Cluster {
        id: cluster_dto.id,

        cluster_state: AtomicU8::new(cluster_dto.cluster_state.into()),
        token_range_replicas: ArcSwap::new(Arc::new(trr)),
        nodes: ArcSwap::new(Arc::new(nodes)),
    };
    client.set_hash(cluster_dto.hash);
    client.set_last_updated_at(Utc::now().timestamp_millis());
    client.cluster.store(Some(Arc::new(cluster)));
}

struct UpdatingGuard {
    client: Arc<Client>,
}

impl Drop for UpdatingGuard {
    fn drop(&mut self) {
        self.client.set_updating(false);
    }
}

pub fn update_cluster(client: Arc<Client>, cluster_dto: ClusterDTO) -> BoxFuture<'static, ()> {
    tracing::debug!("update cluster");
    async move {
        if client
            .is_updating
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            tracing::debug!("already updating");
            return;
        }
        let _guard = UpdatingGuard {
            client: client.clone(),
        };
        let opt_cluster = client.cluster.load_full();
        match opt_cluster {
            Some(cluster) => {
                tracing::debug!("updating cluster");
                let existing = cluster.token_range_replicas.load_full();
                let mut existing_nodes = HashMap::new();
                for trr in existing.iter() {
                    for node in &trr.replicas {
                        existing_nodes.insert(node.id, node.clone());
                    }
                }
                let trr = get_token_range_replicas(
                    client.clone(),
                    cluster_dto.token_range_replicas,
                    existing_nodes,
                )
                .await;
                cluster.token_range_replicas.store(Arc::new(trr));
                client.set_hash(cluster_dto.hash);
                client.set_last_updated_at(Utc::now().timestamp_millis());
            }
            None => init(client.clone(), cluster_dto).await,
        }
    }
    .boxed()
}

pub async fn get_token_range_replicas(
    client: Arc<Client>,
    trr_dto: Vec<TokenRangeReplicasDTO>,
    existing_nodes: HashMap<u64, Arc<Node>>,
) -> Vec<TokenRangeReplicas> {
    let mut nodes_map = existing_nodes;
    let mut token_range_replicas = Vec::new();

    for trr_dto in trr_dto.iter() {
        let mut replicas = Vec::new();
        for node_dto in trr_dto.replicas.iter() {
            let node = if let Some(existing) = nodes_map.get(&node_dto.id) {
                existing.clone()
            } else {
                tracing::debug!("sending connect req to node : {:?}", node_dto.id);
                let pool = AsyncPool::connect(
                    client.clone(),
                    client.cons_count,
                    &node_dto.address.host,
                    node_dto.address.port,
                    client.access_id.clone(),
                    client.access_token.clone(),
                )
                .await;
                let node = Arc::new(Node {
                    id: node_dto.id,
                    address: node_dto.address.to_owned(),
                    pool: Arc::new(ArcSwap::new(Arc::new(pool))),
                });
                nodes_map.insert(node.id, node.clone());
                node
            };
            replicas.push(node);
        }
        let trr = TokenRangeReplicas {
            token_range: trr_dto.token_range.to_owned(),
            replicas,
        };
        token_range_replicas.push(trr);
    }
    token_range_replicas.sort_by(|a, b| a.token_range.end.cmp(&b.token_range.end));
    token_range_replicas
}
