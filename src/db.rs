use crate::memtable::MemTable;
use crate::Result;

/// The main database handle.
#[derive(Debug)]
pub struct Db {
    memtable: MemTable,
}

impl Db {
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self {
            memtable: MemTable::new(),
        })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.memtable.get(key).map(|value| value.to_vec()))
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.memtable.put(key, value);
        Ok(())
    }

    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.memtable.delete(key);
        Ok(())
    }
}
