use chrono::Utc;

use crate::{commands::command_ops::counter::counter_req::CounterReq, util::clock};

impl CounterReq {
    pub fn put(value: i64, ttl: u32) -> CounterReq {
        CounterReq::Put {
            timestamp: Utc::now().timestamp_micros(),

            value: value,
            ttl: if ttl != 0 {
                clock::get_current_secs() + ttl
            } else {
                0
            },
        }
    }
    pub fn get() -> CounterReq {
        CounterReq::Get
    }

    pub fn del() -> CounterReq {
        CounterReq::Del {
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn inc() -> CounterReq {
        CounterReq::Inc {
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn dec() -> CounterReq {
        CounterReq::Dec {
            timestamp: Utc::now().timestamp_micros(),
        }
    }
}
