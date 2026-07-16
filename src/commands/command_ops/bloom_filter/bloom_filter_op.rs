use chrono::Utc;

use crate::commands::command_ops::bloom_filter::{
    bloom_filter_config::BloomFilterConfig,
    bloom_filter_req::{BloomFilterCfgReq, BloomFilterReq},
};

impl BloomFilterReq {
    pub fn add(config_key: String) -> BloomFilterReq {
        BloomFilterReq::Add {
            config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }
    pub fn is_consumed(config_key: String) -> BloomFilterReq {
        BloomFilterReq::IsConsumed { config_key }
    }

    pub fn put_cfg(config_key: String, config: BloomFilterConfig) -> BloomFilterCfgReq {
        BloomFilterCfgReq::PutConfig {
            config_key,
            config: config,
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn get_cfg(config_key: String) -> BloomFilterCfgReq {
        BloomFilterCfgReq::GetConfig { config_key }
    }
    pub fn del_cfg(config_key: String) -> BloomFilterCfgReq {
        BloomFilterCfgReq::DelConfig {
            config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }
}
