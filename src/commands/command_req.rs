use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

use tokio::io::{self, AsyncBufRead};

use crate::{
    cluster::client::Client,
    commands::command_ops::{
        bloom_filter::bloom_filter_req::{BloomFilterCfgReq, BloomFilterReq},
        counter::counter_req::CounterReq,
        rate_limiter::rate_limiter_req::{RateLimiterCfgReq, RateLimiterReq},
        text::text_req::TextReq,
        txt_arr::txt_arr_req::TxtArrReq,
    },
    util::parser,
};

#[derive(Debug, PartialEq)]
pub enum ReqType {
    Client = 1,
    Replica = 2,
    // Storage = 3,
    //  Server = 4,
    None = 255,
}

impl From<u8> for ReqType {
    fn from(value: u8) -> Self {
        match value {
            1 => ReqType::Client,
            2 => ReqType::Replica,
            // 3 => ReqType::Storage,
            // 4 => ReqType::Server,
            _ => ReqType::None,
        }
    }
}

impl From<&ReqType> for u8 {
    fn from(value: &ReqType) -> Self {
        match value {
            ReqType::Client => 1,
            ReqType::Replica => 2,
            // ReqType::Storage => 3,
            // ReqType::Server => 4,
            ReqType::None => 255,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommandType {
    Cfg = 1,
    Data = 2,

    None = 255,
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value {
            1 => CommandType::Cfg,
            2 => CommandType::Data,
            _ => CommandType::None,
        }
    }
}

impl From<&CommandType> for u8 {
    fn from(value: &CommandType) -> Self {
        match value {
            CommandType::Cfg => 1,
            CommandType::Data => 2,
            CommandType::None => 255,
        }
    }
}

#[derive(Debug)]
pub struct ReqHeader {
    pub version: u8,
    pub req_type: ReqType,
    pub command_type: CommandType,

    pub meta_data: Option<ReqMetaData>,
}

#[derive(Debug)]
pub struct ReqMetaData {
    pub client_id: uuid::Uuid,
    pub hash: u64,
    pub server_meta_last_fetch_time: i64,
}

#[derive(Debug, Clone)]
pub enum CommandReq {
    Text(TextReq),
    Counter(CounterReq),
    BloomFilter(BloomFilterReq),
    RateLimiter(RateLimiterReq),
    TxtArr(TxtArrReq),
}

#[derive(Debug, Clone)]
pub struct KeyReq {
    pub key: String,
    pub req: CommandReq,
}

#[derive(Debug, Clone)]
pub enum Req {
    Command(KeyReq),
    Cfg(CfgReq),
    // Server(ServerRequest),
}

impl ReqSerializer for Req {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]> {
        match self {
            Req::Command(command_req) => command_req.serialize(uuid, hash, timestamp),
            Req::Cfg(cfg_req) => cfg_req.serialize(uuid, hash, timestamp),
            // Req::Server(server_req) => server_req.serialize(uuid, hash, timestamp),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CfgReq {
    //  Text(TextCfgReq),
    // Counter(CounterCfgReq),
    BloomFilter(BloomFilterCfgReq),
    RateLimiter(RateLimiterCfgReq),
}

impl CmdParser for CommandReq {
    async fn parse<R>(reader: &mut R, client: Arc<Client>) -> io::Result<Self>
    where
        R: AsyncBufRead + Unpin + Send,
    {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Unsupported parse for Req",
        ))
    }
}

impl CommandSerailizer for KeyReq {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]> {
        let mut buf = Vec::new();
        parser::append_req_header(&mut buf, CommandType::Data, uuid, hash, timestamp);
        parser::append_small_string(&mut buf, &self.key);
        match &self.req {
            CommandReq::Text(text_req) => {
                text_req.serialize(&mut buf);
            }
            CommandReq::Counter(counter_req) => {
                counter_req.serialize(&mut buf);
            }
            CommandReq::BloomFilter(bloom_filter_req) => {
                bloom_filter_req.serialize(&mut buf);
            }
            CommandReq::RateLimiter(rate_limiter_req) => {
                rate_limiter_req.serialize(&mut buf);
            }
            CommandReq::TxtArr(txt_arr_req) => {
                txt_arr_req.serialize(&mut buf);
            }
        }
        Box::from(buf)
    }
}

impl CfgSerailizer for CfgReq {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]> {
        let mut buf = Vec::new();

        parser::append_req_header(&mut buf, CommandType::Cfg, uuid, hash, timestamp);

        match self {
            // CfgReq::Text(text_req) => {
            //     text_req.serialize(&mut buf);
            // }
            // CfgReq::Counter(counter_req) => {
            //     counter_req.serialize(&mut buf);
            // }
            CfgReq::BloomFilter(bloom_filter_req) => {
                bloom_filter_req.serialize(&mut buf);
            }
            CfgReq::RateLimiter(rate_limiter_req) => {
                rate_limiter_req.serialize(&mut buf);
            }
        }
        Box::from(buf)
    }
}

pub trait CmdParser: Sized {
    fn parse<R>(
        reader: &mut R,
        client: Arc<Client>,
    ) -> impl std::future::Future<Output = io::Result<Self>> + Send
    where
        R: AsyncBufRead + Unpin + Send;
}
pub trait CommandSerailizer {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]>;
}

pub trait CfgSerailizer {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]>;
}

pub trait ServerSerailizer {
    fn serialize(&self) -> Box<[u8]>;
}

pub trait ResParser: Sized {
    async fn parse<R>(reader: &mut R) -> io::Result<Self>
    where
        R: AsyncBufRead + Unpin;
}

pub trait CommandReqSerializer {
    fn serialize(&self, buf: &mut Vec<u8>);
}

pub trait ReqSerializer {
    fn serialize(&self, uuid: uuid::Uuid, hash: u64, timestamp: i64) -> Box<[u8]>;
}
