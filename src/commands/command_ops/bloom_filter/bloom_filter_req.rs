use crate::commands::command_ops::bloom_filter::bloom_filter_config::BloomFilterConfig;
use crate::commands::command_req::CommandReqSerializer;
use crate::util::parser::{self};

#[derive(Debug, Clone)]
pub enum BloomFilterReq {
    Add {
        config_key: String,
        //value: String,
        timestamp: i64,
    },

    IsConsumed {
        config_key: String,
        //value: String,
    },
}

impl CommandReqSerializer for BloomFilterReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "BLM_FLTR");
        match self {
            BloomFilterReq::Add {
                config_key,
                // value,
                timestamp,
            } => {
                parser::append_small_string(buf, "ADD");
                parser::append_small_string(buf, config_key);
                // parser::append_small_string(buf, value);
                parser::append_i64(buf, *timestamp);
            }

            BloomFilterReq::IsConsumed { config_key } => {
                parser::append_small_string(buf, "IS_CONSUMED");
                parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, value);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BloomFilterCfgReq {
    PutConfig {
        config_key: String,
        config: BloomFilterConfig,
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

impl CommandReqSerializer for BloomFilterCfgReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "BLM_FLTR");
        match self {
            BloomFilterCfgReq::PutConfig {
                config_key,
                config,
                timestamp,
            } => {
                parser::append_small_string(buf, "PUT_CFG");
                parser::append_small_string(buf, config_key);
                parser::append_i64(buf, config.item_count);
                parser::append_i64(buf, *timestamp);
            }
            BloomFilterCfgReq::GetConfig { config_key } => {
                parser::append_small_string(buf, "GET_CFG");
                parser::append_small_string(buf, config_key);
            }

            BloomFilterCfgReq::DelConfig {
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
