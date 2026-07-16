use std::sync::Arc;

use arc_swap::ArcSwap;
use serde::{Deserialize, Serialize};

use crate::{cluster::ServerAddress, pool::async_pool::AsyncPool};

#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub address: ServerAddress,
    pub pool: Arc<ArcSwap<AsyncPool>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeDTO {
    pub id: u64,
    pub address: ServerAddress,
}

impl std::fmt::Display for NodeDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeDTO {}", self.id)
    }
}
