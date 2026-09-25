use crate::Result;

/// Write-ahead log for durability.
#[derive(Debug, Default)]
pub struct Wal;

impl Wal {
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self)
    }

    pub fn append(&mut self, _key: &[u8], _value: Option<&[u8]>) -> Result<()> {
        Ok(())
    }

    pub fn sync(&self) -> Result<()> {
        Ok(())
    }
}
