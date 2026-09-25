use crate::Result;

/// Writes sorted key-value runs to SSTable files.
#[derive(Debug, Default)]
pub struct SstableWriter;

impl SstableWriter {
    pub fn create(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self)
    }

    pub fn add(&mut self, _key: &[u8], _value: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn finish(self) -> Result<()> {
        Ok(())
    }
}
