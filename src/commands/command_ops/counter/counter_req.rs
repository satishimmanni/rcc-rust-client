use crate::{commands::command_req::CommandReqSerializer, util::parser};

#[derive(Debug, Clone)]
pub enum CounterReq {
    Put {
        //config_key: String,
        //key: String,
        value: i64,
        ttl: u32,
        timestamp: i64,
    },
    // Store {
    //     config_key: String,
    //     key: String,
    //     value: i64,
    //     timestamp: i64,
    // },
    Get, // {
    // config_key: String,
    //key: String,
    //},
    Inc {
        //config_key: String,
        //key: String,
        timestamp: i64,
    },

    Dec {
        // config_key: String,
        //key: String,
        timestamp: i64,
    },

    Del {
        // config_key: String,
        //key: String,
        timestamp: i64,
    },
}

impl CommandReqSerializer for CounterReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "CNTR");
        match self {
            CounterReq::Put {
                // config_key,
                // key,
                value,
                ttl,
                timestamp,
            } => {
                parser::append_small_string(buf, "PUT");
                //parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
                parser::append_i64(buf, *value);
                parser::append_u32(buf, *ttl);
                parser::append_i64(buf, *timestamp);
            }

            CounterReq::Get => {
                parser::append_small_string(buf, "GET");
                //  parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
            }
            CounterReq::Inc {
                // config_key,
                //key,
                timestamp,
            } => {
                parser::append_small_string(buf, "INC");
                //parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
                parser::append_i64(buf, *timestamp);
            }
            CounterReq::Dec {
                //  config_key,
                //key,
                timestamp,
            } => {
                parser::append_small_string(buf, "DEC");
                //  parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
                parser::append_i64(buf, *timestamp);
            }
            CounterReq::Del {
                //   config_key,
                // key,
                timestamp,
            } => {
                parser::append_small_string(buf, "DEL");
                // parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
                parser::append_i64(buf, *timestamp);
            }
        }
    }
}
