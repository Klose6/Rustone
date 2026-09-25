use std::collections::BTreeMap;

/// In-memory sorted table for recent writes.
#[derive(Debug, Default)]
pub struct MemTable {
    data: BTreeMap<Vec<u8>, Option<Vec<u8>>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        self.data.insert(key.to_vec(), Some(value.to_vec()));
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.data.insert(key.to_vec(), None);
    }

    pub fn get(&self, key: &[u8]) -> Option<Option<&[u8]>> {
        self.data
            .get(key)
            .map(|value| value.as_ref().map(|v| v.as_slice()))
    }
}
