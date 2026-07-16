use crate::{commands::command_req::CommandReqSerializer, util::parser};

#[derive(Debug, Clone)]
pub enum TxtArrReq {
    Put {
        timestamp: i64,
        value: Vec<String>,
        ttl: u32,
    },
    Get,
    Del {
        timestamp: i64,
    },
}

impl CommandReqSerializer for TxtArrReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "TXT_ARR");
        match self {
            TxtArrReq::Put {
                timestamp,
                value,
                ttl,
            } => {
                parser::append_small_string(buf, "PUT");
                parser::append_txt_array(buf, &value);
                parser::append_u32(buf, *ttl);
                parser::append_i64(buf, *timestamp);
            }
            TxtArrReq::Del { timestamp } => {
                parser::append_small_string(buf, "DEL");
                parser::append_i64(buf, *timestamp);
            }
            TxtArrReq::Get => {
                parser::append_small_string(buf, "GET");
            }
        }
    }
}
