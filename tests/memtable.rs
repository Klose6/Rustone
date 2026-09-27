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

#[test]
fn iter_skips_tombstones_and_sorts_keys() {
    let mut table = MemTable::new();
    table.put(b"c", b"3");
    table.put(b"a", b"1");
    table.put(b"b", b"2");
    table.delete(b"b");

    let entries: Vec<_> = table
        .iter()
        .map(|(key, value)| (key.to_vec(), value.to_vec()))
        .collect();

    assert_eq!(
        entries,
        vec![(b"a".to_vec(), b"1".to_vec()), (b"c".to_vec(), b"3".to_vec())]
    );
}
