use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TokenRange {
    pub start: u64, //excludes
    pub end: u64,   //includes
}

impl std::fmt::Display for TokenRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TR {} - {}", self.start, self.end)
    }
}
