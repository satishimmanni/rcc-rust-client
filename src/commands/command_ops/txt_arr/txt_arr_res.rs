use tokio::io::AsyncBufRead;

use std::io::{Error, ErrorKind, Result};

use crate::{
    commands::{command_req::ResParser, command_status::CommandStatus},
    util::parser,
};

#[derive(Debug)]
pub enum TxtArrRes {
    Put {
        status: CommandStatus,
    },
    Get {
        value: Vec<String>,
        status: CommandStatus,
    },
    Del {
        value: Vec<String>,
        status: CommandStatus,
    },
}

impl ResParser for TxtArrRes {
    async fn parse<R>(reader: &mut R) -> Result<TxtArrRes>
    where
        R: AsyncBufRead + Unpin,
    {
        let cmd = parser::read_small_string(reader).await?;
        //let cmd = std::str::from_utf8(&cmd_bytes).unwrap().to_uppercase();

        match cmd.as_str() {
            "PUT" => Ok(TxtArrRes::Put {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET" => Ok(TxtArrRes::Get {
                value: parser::read_txt_array(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL" => Ok(TxtArrRes::Del {
                value: parser::read_txt_array(reader).await?,
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Unknown txt arr res command",
            )),
        }
    }
}
