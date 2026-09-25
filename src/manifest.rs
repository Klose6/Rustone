use crate::Result;

/// On-disk metadata log tracking database file changes.
#[derive(Debug, Default)]
pub struct Manifest;

impl Manifest {
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self)
    }

    pub fn record_new_file(&mut self, _file_number: u64) -> Result<()> {
        Ok(())
    }
}
