use crate::Result;

/// Reads key-value entries from SSTable files.
#[derive(Debug, Default)]
pub struct SstableReader;

impl SstableReader {
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self)
    }

    pub fn get(&self, _key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}
