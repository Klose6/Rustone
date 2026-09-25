/// A fixed-size block of key-value entries within an SSTable.
#[derive(Debug, Default)]
pub struct Block {
    data: Vec<u8>,
}

impl Block {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}
