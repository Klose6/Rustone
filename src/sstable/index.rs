/// Index block mapping keys to data block offsets.
#[derive(Debug, Default)]
pub struct Index {
    entries: Vec<(Vec<u8>, u64)>,
}

impl Index {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, key: Vec<u8>, offset: u64) {
        self.entries.push((key, offset));
    }

    pub fn entries(&self) -> &[(Vec<u8>, u64)] {
        &self.entries
    }
}
