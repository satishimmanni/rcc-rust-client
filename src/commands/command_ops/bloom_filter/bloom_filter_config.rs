#[derive(Debug,Clone)]
pub struct BloomFilterConfig {
    pub item_count: i64,
}

impl BloomFilterConfig {
    pub fn to_str(&self) -> String {
        self.item_count.to_string()
    }
}
