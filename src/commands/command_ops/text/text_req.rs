use crate::{commands::command_req::CommandReqSerializer, util::parser};

#[derive(Debug, Clone)]
pub enum TextReq {
    // *6\r\n$3\r\nPUT\r\n$3\r\nCFG\r\n:123\r\n$1\r\nK\r\n$1\r\nV\r\n:3600\r\n
    Put {
        // config_key: String,
        timestamp: i64,
        //key: String,
        value: String,
        ttl: u32,
    },
    Get, // config_key: String,
    //key: String,
    Del {
        //config_key: String,
        timestamp: i64,
        //key: String,
    },
}

impl CommandReqSerializer for TextReq {
    fn serialize(&self, buf: &mut Vec<u8>) {
        parser::append_small_string(buf, "TXT");
        match self {
            TextReq::Put {
                //config_key,
                timestamp,
                //key,
                value,
                ttl,
            } => {
                // Command header: *6 elements (TXT, PUT, and 4 args)
                //buf.extend_from_slice(b"*7\r\n");
                //parser::append_command(buf);

                parser::append_small_string(buf, "PUT");
                // parser::append_small_string(buf, config_key);
                // parser::append_small_string(buf, key);
                parser::append_bulk_string(buf, value);
                parser::append_u32(buf, *ttl);
                parser::append_i64(buf, *timestamp);
            }
            TextReq::Get => {
                //buf.extend_from_slice(b"*4\r\n");

                parser::append_small_string(buf, "GET");
                // parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
            }

            TextReq::Del {
                // config_key,
                timestamp,
                //key,
            } => {
                //buf.extend_from_slice(b"*4\r\n");

                parser::append_small_string(buf, "DEL");
                // parser::append_small_string(buf, config_key);
                //parser::append_small_string(buf, key);
                parser::append_i64(buf, *timestamp);
            }
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum TextCfgReq {
//     // *3\r\n$9\r\nPUTCONFIG\r\n$3\r\nKEY\r\n:1711550000\r\n
//     PutConfig {
//         config_key: String,
//         capacity: i64,
//         timestamp: i64,
//     },
//     GetConfig {
//         config_key: String,
//     },
//     DelConfig {
//         config_key: String,
//         timestamp: i64,
//     },
// }

// impl CommandReqSerializer for TextCfgReq {
//     fn serialize(&self, buf: &mut Vec<u8>) {
//         parser::append_small_string(buf, "TXT");
//         match self {
//             TextCfgReq::PutConfig {
//                 config_key,
//                 capacity,
//                 timestamp,
//             } => {
//                 //buf.extend_from_slice(b"*4\r\n");

//                 parser::append_small_string(buf, "PUT_CFG");
//                 parser::append_small_string(buf, config_key);
//                 parser::append_i64(buf, *capacity);
//                 parser::append_i64(buf, *timestamp);
//             }
//             TextCfgReq::GetConfig { config_key } => {
//                 //buf.extend_from_slice(b"*4\r\n");

//                 parser::append_small_string(buf, "GET_CFG");
//                 parser::append_small_string(buf, config_key);
//             }
//             TextCfgReq::DelConfig {
//                 config_key,
//                 timestamp,
//             } => {
//                 //buf.extend_from_slice(b"*4\r\n");

//                 parser::append_small_string(buf, "DEL_CFG");
//                 parser::append_small_string(buf, config_key);
//                 parser::append_i64(buf, *timestamp);
//             }
//         }
//     }
// }
