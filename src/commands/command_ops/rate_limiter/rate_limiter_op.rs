use chrono::Utc;

use crate::commands::command_ops::rate_limiter::{
    rate_limiter_config::RateLimiterConfig,
    rate_limiter_req::{RateLimiterCfgReq, RateLimiterReq},
};

impl RateLimiterReq {
    pub fn consume(config_key: String, tokens: u64) -> RateLimiterReq {
        RateLimiterReq::Consume {
            config_key,
            // key,
            tokens: tokens,
        }
    }

    pub fn put_cfg(config_key: String, config: RateLimiterConfig) -> RateLimiterCfgReq {
        RateLimiterCfgReq::PutConfig {
            config_key,
            config,
            timestamp: Utc::now().timestamp_micros(),
        }
    }

    pub fn get_cfg(config_key: String) -> RateLimiterCfgReq {
        RateLimiterCfgReq::GetConfig { config_key }
    }
    pub fn del_cfg(config_key: String) -> RateLimiterCfgReq {
        RateLimiterCfgReq::DelConfig {
            config_key,
            timestamp: Utc::now().timestamp_micros(),
        }
    }
}
