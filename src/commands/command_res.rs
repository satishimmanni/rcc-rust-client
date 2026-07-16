use tokio::io::AsyncBufRead;

use crate::{
    cluster::client::{self, Client},
    commands::{
        command_ops::{
            bloom_filter::bloom_filter_res::BloomFilterRes, counter::counter_res::CounterRes,
            rate_limiter::rate_limiter_res::RateLimiterRes, text::text_res::TextRes,
            txt_arr::txt_arr_res::TxtArrRes,
        },
        command_req::{CmdParser, ResParser},
        command_status::CommandStatus,
    },
    util::parser,
};
use std::{
    io::{Error, ErrorKind, Result},
    sync::Arc,
};

#[derive(Debug, PartialEq)]
pub enum ResType {
    Cfg = 1,
    Data = 2,
    // Server = 3,
    None = 255,
}

impl From<u8> for ResType {
    fn from(value: u8) -> Self {
        match value {
            1 => ResType::Cfg,
            2 => ResType::Data,
            //   3 => ResType::Server,
            _ => ResType::None,
        }
    }
}

impl From<&ResType> for u8 {
    fn from(value: &ResType) -> Self {
        match value {
            ResType::Cfg => 1,
            ResType::Data => 2,
            // ResType::Server => 3,
            ResType::None => 255,
        }
    }
}

#[derive(Debug)]
pub enum CommandRes {
    Text(TextRes),
    Counter(CounterRes),
    BloomFilter(BloomFilterRes),
    RateLimiter(RateLimiterRes),
    Error(ErrorRes),
    Response(Response),
    //Server(ServerRes),
    TxtArr(TxtArrRes),
}

impl CmdParser for CommandRes {
    async fn parse<R>(reader: &mut R, client: Arc<Client>) -> Result<CommandRes>
    where
        R: AsyncBufRead + Unpin + Send,
    {
        let header = parser::read_res_header(reader).await?;
        // tracing::debug!("header: {:?}", header);
        // if header.0 == ResType::Server {
        //     return Ok(CommandRes::Server(ServerRes::parse(reader).await?));
        // }
        //  tracing::debug!("header: {:?}", header);
        if let Some(cluster) = header.1 {
            if client.get_hash() != cluster.hash
                && client.get_last_updated_at() < cluster.last_updated_at
            {
                tracing::debug!("update cluster from COMMAND res parser");
                client::update_cluster(client, cluster).await;
            }
        }

        let cmd_type = parser::read_small_string(reader).await?;
        // tracing::debug!("cmd_type: {:?}", cmd_type);
        // let cmd = std::str::from_utf8(&cmd_type).unwrap();
        match cmd_type.as_str() {
            "TXT" => Ok(CommandRes::Text(TextRes::parse(reader).await?)),
            "CNTR" => Ok(CommandRes::Counter(CounterRes::parse(reader).await?)),
            "BLM_FLTR" => Ok(CommandRes::BloomFilter(
                BloomFilterRes::parse(reader).await?,
            )),
            "RT_LMTR" => Ok(CommandRes::RateLimiter(
                RateLimiterRes::parse(reader).await?,
            )),
            "ERR" => Ok(CommandRes::Error(ErrorRes::parse(reader).await?)),
            "RESP" => Ok(CommandRes::Response(Response::parse(reader).await?)),
            "TXT_ARR" => Ok(CommandRes::TxtArr(TxtArrRes::parse(reader).await?)),

            _ => Err(Error::new(ErrorKind::InvalidData, "invalid commandres")),
        }
    }
}
// impl CmdSerailizer for CommandRes{

//     async fn serialize(&self) -> Box<[u8]> {
//         match self {
//             CommandRes::Text(text_res) => text_res.serialize().await,
//             CommandRes::Counter(counter_res) => counter_res.serialize().await,
//             CommandRes::BloomFilter(bloom_filter_res) => bloom_filter_res.serialize().await,
//             CommandRes::RateLimiter(rate_limiter_res) => rate_limiter_res.serialize().await,
//             CommandRes::Error(error_res) => error_res.serialize().await,
//             CommandRes::Response(response) => response.serialize().await,
//         }
//     }
// }

#[derive(Debug)]
pub struct Response {
    pub status: ServerResponse,
}

#[derive(Debug, Clone)]
pub enum ServerResponse {
    Booting = 1,
    Error = 8,
}

impl From<u8> for ServerResponse {
    fn from(value: u8) -> Self {
        match value {
            1 => ServerResponse::Booting,

            _ => ServerResponse::Error,
        }
    }
}

impl From<ServerResponse> for u8 {
    fn from(value: ServerResponse) -> Self {
        match value {
            ServerResponse::Booting => 1,
            ServerResponse::Error => 8,
        }
    }
}

impl ResParser for Response {
    async fn parse<R>(reader: &mut R) -> tokio::io::Result<Self>
    where
        R: AsyncBufRead + Unpin,
    {
        Ok(Response {
            status: ServerResponse::from(parser::read_u8(reader).await?),
        })
    }
}

#[derive(Debug)]
pub struct ErrorRes {
    pub message: String,
    pub status: CommandStatus,
}

impl ResParser for ErrorRes {
    async fn parse<R>(reader: &mut R) -> tokio::io::Result<Self>
    where
        R: AsyncBufRead + Unpin,
    {
        Ok(ErrorRes {
            message: parser::read_small_string(reader).await?,
            status: CommandStatus::from(parser::read_u8(reader).await?),
        })
    }
}
