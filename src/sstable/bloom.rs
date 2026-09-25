/// Bloom filter for fast negative lookups.
#[derive(Debug, Default)]
pub struct BloomFilter;

impl BloomFilter {
    pub fn new(_expected_items: usize, _false_positive_rate: f64) -> Self {
        Self
    }

    pub fn might_contain(&self, _key: &[u8]) -> bool {
        true
    }

    pub fn insert(&mut self, _key: &[u8]) {}
}
