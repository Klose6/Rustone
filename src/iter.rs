use crate::Result;

/// Iterator over key-value entries in storage order.
pub trait Iterator {
    fn seek_to_first(&mut self) -> Result<()>;
    fn valid(&self) -> bool;
    fn key(&self) -> Option<&[u8]>;
    fn value(&self) -> Option<&[u8]>;
    fn next(&mut self) -> Result<()>;
}

/// Merging iterator over multiple sorted sources.
#[derive(Debug, Default)]
pub struct MergingIterator;

impl MergingIterator {
    pub fn new() -> Self {
        Self
    }
}

impl Iterator for MergingIterator {
    fn seek_to_first(&mut self) -> Result<()> {
        Ok(())
    }

    fn valid(&self) -> bool {
        false
    }

    fn key(&self) -> Option<&[u8]> {
        None
    }

    fn value(&self) -> Option<&[u8]> {
        None
    }

    fn next(&mut self) -> Result<()> {
        Ok(())
    }
}
