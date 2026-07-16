use crate::commands::command_req::CommandReqSerializer;
use crate::util::parser;

use crate::commands::command_ops::rate_limiter::rate_limiter_config::RateLimiterConfig;

#[derive(Debug, Clone)]
pub enum RateLimiterReq {
    Consume {
        config_key: String,
        // key: String,
        tokens: u64,
    },
}

impl CommandReqSerializer for RateLimiterReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "RT_LMTR");
        match self {
            RateLimiterReq::Consume {
                config_key,
                ///  key,
                tokens,
            } => {
                parser::append_small_string(buf, "CONSUME");
                parser::append_small_string(buf, config_key);
                // parser::append_small_string(buf, key);
                parser::append_u64(buf, *tokens);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RateLimiterCfgReq {
    PutConfig {
        config_key: String,
        config: RateLimiterConfig,
        timestamp: i64,
    },

    GetConfig {
        config_key: String,
    },

    DelConfig {
        config_key: String,
        timestamp: i64,
    },
}

impl CommandReqSerializer for RateLimiterCfgReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "RT_LMTR");
        match self {
            RateLimiterCfgReq::PutConfig {
                config_key,
                config,
                timestamp,
            } => {
                parser::append_small_string(buf, "PUT_CFG");
                parser::append_small_string(buf, config_key);
                parser::append_u64(buf, config.interval_sec);
                parser::append_u64(buf, config.max_tokens);
                parser::append_i64(buf, *timestamp);
            }
            RateLimiterCfgReq::GetConfig { config_key } => {
                parser::append_small_string(buf, "GET_CFG");
                parser::append_small_string(buf, config_key);
            }

            RateLimiterCfgReq::DelConfig {
                config_key,
                timestamp,
            } => {
                parser::append_small_string(buf, "DEL_CFG");
                parser::append_small_string(buf, config_key);
                parser::append_i64(buf, *timestamp);
            }
        }
    }
}
