#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    pub interval_sec: u64,
    pub max_tokens: u64,
}

impl RateLimiterConfig {
    pub fn to_str(&self) -> String {
        format!("{}|{}", self.interval_sec, self.max_tokens)
    }
}
