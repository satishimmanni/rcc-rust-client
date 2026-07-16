use tokio::io::AsyncBufRead;

use crate::{
    commands::{
        command_ops::rate_limiter::rate_limiter_config::RateLimiterConfig, command_req::ResParser,
        command_status::CommandStatus,
    },
    util::parser,
};
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub enum RateLimiterRes {
    PutConfig {
        status: CommandStatus,
    },

    GetConfig {
        config: RateLimiterConfig,
        status: CommandStatus,
    },

    Consume {
        status: CommandStatus,
    },

    DelConfig {
        config: RateLimiterConfig,
        status: CommandStatus,
    },
}

impl ResParser for RateLimiterRes {
    async fn parse<R>(reader: &mut R) -> Result<RateLimiterRes>
    where
        R: AsyncBufRead + Unpin,
    {
        let cmd = parser::read_small_string(reader).await?;
        // let cmd = std::str::from_utf8(&cmd_bytes).unwrap().to_uppercase();

        match cmd.as_str() {
            "PUT_CFG" => Ok(RateLimiterRes::PutConfig {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "GET_CFG" => Ok(RateLimiterRes::GetConfig {
                config: RateLimiterConfig {
                    interval_sec: parser::read_u64(reader).await?,
                    max_tokens: parser::read_u64(reader).await?,
                },
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "CONSUME" => Ok(RateLimiterRes::Consume {
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            "DEL_CFG" => Ok(RateLimiterRes::DelConfig {
                config: RateLimiterConfig {
                    interval_sec: parser::read_u64(reader).await?,
                    max_tokens: parser::read_u64(reader).await?,
                },
                status: CommandStatus::from(parser::read_u8(reader).await?),
            }),
            _ => Err(Error::new(ErrorKind::InvalidData, "Unknown rt res command")),
        }
    }
}
