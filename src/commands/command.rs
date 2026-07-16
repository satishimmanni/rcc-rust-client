use crate::commands::command_req::Req;

#[derive(Debug, Clone)]
pub struct RetryReq {
    pub command: Req,
    pub retry_count: u8,
}
