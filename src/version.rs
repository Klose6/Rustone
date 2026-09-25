use crate::Result;

/// A snapshot of the set of SSTables visible at a point in time.
#[derive(Debug, Default)]
pub struct Version {
    files: Vec<u64>,
}

impl Version {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_file(&mut self, file_number: u64) {
        self.files.push(file_number);
    }

    pub fn files(&self) -> &[u64] {
        &self.files
    }

    pub fn save(_path: impl AsRef<std::path::Path>, version: &Version) -> Result<()> {
        let _ = version;
        Ok(())
    }
}
