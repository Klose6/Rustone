use crate::Result;

/// Compacts SSTable levels to reclaim space and improve read performance.
#[derive(Debug, Default)]
pub struct Compactor;

impl Compactor {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self) -> Result<()> {
        Ok(())
    }
}
