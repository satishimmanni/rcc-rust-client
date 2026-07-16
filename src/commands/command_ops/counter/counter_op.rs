use chrono::Utc;

use crate::commands::command_ops::counter::counter_req::CounterReq;

impl CounterReq {
    // pub fn put_cfg(config_key: String, capacity: i64) -> CounterCfgReq {
    //     CounterCfgReq::PutConfig {
    //         config_key,
    //         capacity,
    //         timestamp: Utc::now().timestamp_micros(),
    //     }
    // }

    // pub fn get_cfg(config_key: String) -> CounterCfgReq {
    //     CounterCfgReq::GetConfig { config_key }
    // }
    // pub fn del_cfg(config_key: String) -> CounterCfgReq {
    //     CounterCfgReq::DelConfig {
    //         config_key,
    //         timestamp: Utc::now().timestamp_micros(),
    //     }
    // }

    pub fn put(value: i64, ttl: u32) -> CounterReq {
        CounterReq::Put {
            // config_key,
            timestamp: Utc::now().timestamp_micros(),

            value: value,
            ttl: if ttl != 0 {
                Utc::now().timestamp() as u32 + ttl
            } else {
                0
            },
        }
    }
    pub fn get() -> CounterReq {
        CounterReq::Get //{ config_key }
    }

    pub fn del() -> CounterReq {
        CounterReq::Del {
            // config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn inc() -> CounterReq {
        CounterReq::Inc {
            // config_key,
            //key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn dec() -> CounterReq {
        CounterReq::Dec {
            //  config_key,
            // key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }
}
