use chrono::Utc;

use crate::{commands::command_ops::text::text_req::TextReq, util::clock};

impl TextReq {
    pub fn put(value: String, ttl: u32) -> TextReq {
        TextReq::Put {
            timestamp: Utc::now().timestamp_micros(),
            value,
            ttl: if ttl != 0 {
                clock::get_current_secs() + ttl
            } else {
                0
            },
        }
    }
    pub fn get() -> TextReq {
        TextReq::Get
    }

    pub fn del() -> TextReq {
        TextReq::Del {
            //config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    // pub fn put_cfg(config_key: String, capacity: i64) -> TextCfgReq {
    //     TextCfgReq::PutConfig {
    //         config_key,
    //         capacity,
    //         timestamp: Utc::now().timestamp_micros(),
    //     }
    // }

    // pub fn get_cfg(config_key: String) -> TextCfgReq {
    //     TextCfgReq::GetConfig { config_key }
    // }
    // pub fn del_cfg(config_key: String) -> TextCfgReq {
    //     TextCfgReq::DelConfig {
    //         config_key,
    //         timestamp: Utc::now().timestamp_micros(),
    //     }
    // }
}
