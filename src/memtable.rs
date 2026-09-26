use std::collections::BTreeMap;

/// In-memory sorted table for recent writes.
#[derive(Debug, Default)]
pub struct MemTable {
    // In Rust, Vec<u8> is owned bytes — it's the standard way to store a byte buffer on the heap.
    data: BTreeMap<Vec<u8>, Option<Vec<u8>>>,
}

impl MemTable {
    pub fn new() -> Self {
        // creating a MemTable with an empty BTreeMap
        Self::default()
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        self.data.insert(key.to_vec(), Some(value.to_vec()));
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.data.insert(key.to_vec(), None);
    }

    /// Returns `Some(value)` if the key exists, `None` if missing or deleted.
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        match self.data.get(key) {
            Some(Some(value)) => Some(value),
            Some(None) | None => None,
        }
    }
}
