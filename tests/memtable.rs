use rustone::memtable::MemTable;

#[test]
fn put_and_get() {
    let mut table = MemTable::new();
    table.put(b"a", b"1");
    assert_eq!(table.get(b"a"), Some(b"1".as_slice()));
}

#[test]
fn delete_tombstones_key() {
    let mut table = MemTable::new();
    table.put(b"a", b"1");
    table.delete(b"a");
    assert_eq!(table.get(b"a"), None);
}
