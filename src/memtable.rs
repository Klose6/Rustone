use std::collections::BTreeMap;

/// In-memory sorted table for recent writes.
#[derive(Debug, Default)]
pub struct MemTable {
    // Vec<u8> is owned bytes — it's the standard way to store a byte buffer on the heap.
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

    /// Returns `Some(value)` if the key exists, `None` if missing or deleted.
    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        match self.data.get(key) {
            Some(Some(value)) => Some(value),
            Some(None) | None => None,
        }
    }

    /// Iterates live entries in sorted key order, skipping tombstones.
    pub fn iter(&self) -> MemTableIter<'_> {
        MemTableIter {
            inner: self.data.iter(),
        }
    }
}

/// Iterator over live entries in a [`MemTable`].
pub struct MemTableIter<'a> {
    inner: std::collections::btree_map::Iter<'a, Vec<u8>, Option<Vec<u8>>>,
}

impl<'a> Iterator for MemTableIter<'a> {
    type Item = (&'a [u8], &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        for (key, value) in self.inner.by_ref() {
            if let Some(value) = value {
                return Some((key.as_slice(), value.as_slice()));
            }
        }
        None
    }
}
