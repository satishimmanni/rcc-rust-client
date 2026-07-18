use std::io::{Error, ErrorKind};

use crate::commands::{
    command_ops::bloom_filter::bloom_filter_config::BloomFilterConfig, command_req::ResParser,
    command_status::CommandStatus,
};

use tokio::io::AsyncBufRead;

use crate::util::parser;
use std::io::Result;

#[derive(Debug)]
pub enum BloomFilterRes {
    PutConfig {
        status: CommandStatus,
    },

    GetConfig {
        config: BloomFilterConfig,
        status: CommandStatus,
    },

    Add {
        // value: String,
        status: CommandStatus,
    },
    Store {
        // value: String,
        status: CommandStatus,
    },

    IsConsumed {
        // value: String,
        status: CommandStatus,
    },

    DelConfig {
        config: BloomFilterConfig,
        status: CommandStatus,
    },
}

impl ResParser for BloomFilterRes {
    async fn parse<R>(reader: &mut R) -> Result<BloomFilterRes>
    where
        R: AsyncBufRead + Unpin,
    {
        let cmd = parser::read_small_string(reader).await?;
        // let cmd = std::str::from_utf8(&cmd_bytes).unwrap().to_uppercase();

        match cmd.as_str() {
            "ADD" => Ok(BloomFilterRes::Add {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "STORE" => Ok(BloomFilterRes::Store {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "IS_CONSUMED" => Ok(BloomFilterRes::IsConsumed {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "PUT_CFG" => Ok(BloomFilterRes::PutConfig {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET_CFG" => Ok(BloomFilterRes::GetConfig {
                config: BloomFilterConfig {
                    item_count: parser::read_u64(reader).await?,
                },
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL_CFG" => Ok(BloomFilterRes::DelConfig {
                config: BloomFilterConfig {
                    item_count: parser::read_u64(reader).await?,
                },
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Unknown BLM FLTR res command",
            )),
        }
    }
}
