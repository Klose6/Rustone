use crate::Result;

/// The main database handle.
#[derive(Debug, Default)]
pub struct Db;

impl Db {
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self> {
        Ok(Self)
    }

    pub fn get(&self, _key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }

    pub fn put(&mut self, _key: &[u8], _value: &[u8]) -> Result<()> {
        Ok(())
    }

    pub fn delete(&mut self, _key: &[u8]) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_and_get_missing_key() {
        let db = Db::open("/tmp/rustone-test").unwrap();
        assert!(matches!(db.get(b"missing"), Ok(None)));
    }

    #[test]
    fn put_and_get_round_trip_is_not_implemented_yet() {
        let mut db = Db::open("/tmp/rustone-test").unwrap();
        db.put(b"key", b"value").unwrap();
        assert!(matches!(db.get(b"key"), Ok(None)));
    }
}
