use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use tokio::io::AsyncBufRead;

use crate::{
    cluster::client::{Client, ClusterDTO},
    commands::command_req::CmdParser,
    util::parser,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Client,
    Server,
    None,
}

impl From<u8> for ConnectionType {
    fn from(value: u8) -> Self {
        match value {
            1 => ConnectionType::Client,
            2 => ConnectionType::Server,
            _ => ConnectionType::None,
        }
    }
}

impl From<&ConnectionType> for u8 {
    fn from(value: &ConnectionType) -> Self {
        match value {
            ConnectionType::Client => 1,
            ConnectionType::Server => 2,
            ConnectionType::None => 255,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectReq {
    pub connection_type: ConnectionType,
    pub client_id: String,
    pub access_token: String,
}

impl CmdParser for ConnectReq {
    async fn parse<R>(reader: &mut R, client: Arc<Client>) -> tokio::io::Result<Self>
    where
        R: AsyncBufRead + Unpin + Send,
    {
        let json_str = parser::read_small_string(reader).await?;
        let connect_req: ConnectReq = serde_json::from_str(&json_str)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;

        Ok(connect_req)
    }
}

impl ConnectReq {
    pub fn serialize(&self) -> Box<[u8]> {
        let mut buf = Vec::new();
        let str = serde_json::to_string(self).unwrap();
        parser::append_small_string(&mut buf, &str);
        Box::from(buf)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRes {
    pub message: String,
}

impl ConnectRes {
    pub async fn parse<R>(reader: &mut R) -> tokio::io::Result<(Self, Option<ClusterDTO>)>
    where
        R: AsyncBufRead + Unpin,
    {
        //  tracing::debug!("reading connect res");
        let header = parser::read_res_header(reader).await?;
        //tracing::debug!("header: {:?}", header);

        let cmd = parser::read_small_string(reader).await?;
        //tracing::debug!("cmd: {:?}", cmd);

        match cmd.as_str() {
            "CONNECT" => {
                let message = parser::read_small_string(reader).await?;
                Ok((ConnectRes { message: message }, header.1))
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid Connect res cmd",
            )),
        }
    }
}
