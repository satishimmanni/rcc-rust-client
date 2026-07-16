use tokio::io::AsyncBufRead;

use std::io::{Error, ErrorKind, Result};

use crate::{
    commands::{command_req::ResParser, command_status::CommandStatus},
    util::parser,
};

#[derive(Debug)]
pub enum TextRes {
    // PutConfig {
    //     status: CommandStatus,
    // },
    // GetConfig {
    //     capacity: i64,
    //     status: CommandStatus,
    // },
    // DelConfig {
    //     status: CommandStatus,
    // },
    Put {
        status: CommandStatus,
    },

    Get {
        value: String,
        status: CommandStatus,
    },

    Del {
        value: String,
        status: CommandStatus,
    },
}

impl ResParser for TextRes {
    async fn parse<R>(reader: &mut R) -> Result<TextRes>
    where
        R: AsyncBufRead + Unpin,
    {
        let cmd = parser::read_small_string(reader).await?;
        // let cmd = std::str::from_utf8(&cmd_bytes).unwrap().to_uppercase();

        match cmd.as_str() {
            "PUT" => Ok(TextRes::Put {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET" => Ok(TextRes::Get {
                value: parser::read_bulk_string(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL" => Ok(TextRes::Del {
                value: parser::read_bulk_string(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            // "PUT_CFG" => Ok(TextRes::PutConfig {
            //     status: CommandStatus::from(parser::read_u8(reader).await?),
            // }),
            // "GET_CFG" => Ok(TextRes::GetConfig {
            //     capacity: parser::read_i64(reader).await?,
            //     status: CommandStatus::from(parser::read_u8(reader).await?),
            // }),
            // "DEL_CFG" => Ok(TextRes::DelConfig {
            //     status: CommandStatus::from(parser::read_u8(reader).await?),
            // }),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Unknown text res command",
            )),
        }
    }
}
