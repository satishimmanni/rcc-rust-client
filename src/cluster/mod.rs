use serde::{Deserialize, Serialize};

pub mod client;
pub mod cluster_state;
pub mod node;
pub mod token_range;
pub mod token_range_replica;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct ServerAddress {
    pub host: String,
    pub port: u16,
    pub region: String,
    pub az: String,
}
