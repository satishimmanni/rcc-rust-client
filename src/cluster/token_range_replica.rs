use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::cluster::{
    node::{Node, NodeDTO},
    token_range::TokenRange,
};

#[derive(Debug)]
pub struct TokenRangeReplicas {
    pub token_range: TokenRange,
    pub replicas: Vec<Arc<Node>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenRangeReplicasDTO {
    pub token_range: TokenRange,
    pub replicas: Vec<NodeDTO>,
}

impl std::fmt::Display for TokenRangeReplicasDTO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let replicas = self
            .replicas
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{} [{}]", self.token_range, replicas)
    }
}
