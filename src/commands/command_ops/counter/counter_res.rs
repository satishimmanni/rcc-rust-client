use std::io::{Error, ErrorKind};

use tokio::io::AsyncBufRead;

use crate::{
    commands::{command_req::ResParser, command_status::CommandStatus},
    util::parser,
};
use std::io::Result;

#[derive(Debug)]
pub enum CounterRes {
    PutConfig { status: CommandStatus },
    GetConfig { status: CommandStatus },
    DelConfig { status: CommandStatus },

    Put { status: CommandStatus },

    Get { value: i64, status: CommandStatus },

    Inc { value: i64, status: CommandStatus },

    Dec { value: i64, status: CommandStatus },

    Del { value: i64, status: CommandStatus },
}

impl ResParser for CounterRes {
    async fn parse<R>(reader: &mut R) -> Result<CounterRes>
    where
        R: AsyncBufRead + Unpin,
    {
        let cmd = parser::read_small_string(reader).await?;
        // let cmd = std::str::from_utf8(&cmd_bytes).unwrap().to_uppercase();

        match cmd.as_str() {
            "PUT" => Ok(CounterRes::Put {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET" => Ok(CounterRes::Get {
                value: parser::read_i64(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "INC" => Ok(CounterRes::Inc {
                value: parser::read_i64(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEC" => Ok(CounterRes::Dec {
                value: parser::read_i64(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL" => Ok(CounterRes::Del {
                value: parser::read_i64(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "PUT_CFG" => Ok(CounterRes::PutConfig {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET_CFG" => Ok(CounterRes::GetConfig {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL_CFG" => Ok(CounterRes::DelConfig {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),

            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Unknown cntr res command",
            )),
        }
    }
}
