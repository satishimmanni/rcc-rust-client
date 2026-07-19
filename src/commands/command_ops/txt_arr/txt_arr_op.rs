use chrono::Utc;

use crate::{commands::command_ops::txt_arr::txt_arr_req::TxtArrReq, util::clock};

impl TxtArrReq {
    pub fn put(value: Vec<String>, ttl: u32) -> TxtArrReq {
        TxtArrReq::Put {
            timestamp: Utc::now().timestamp_micros(),
            value,
            ttl: if ttl != 0 {
                clock::get_current_secs() + ttl
            } else {
                0
            },
        }
    }
    pub fn get() -> TxtArrReq {
        TxtArrReq::Get
    }

    pub fn del() -> TxtArrReq {
        TxtArrReq::Del {
            //config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }
}
