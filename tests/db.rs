use rustone::Db;

#[test]
fn opens_a_database() {
    let db = Db::open("/tmp/rustone-integration-test").unwrap();
    assert!(db.get(b"key").unwrap().is_none());
}

#[test]
fn get_missing_key() {
    let db = Db::open("/tmp/rustone-test").unwrap();
    assert_eq!(db.get(b"missing").unwrap(), None);
}

#[test]
fn put_and_get() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.put(b"key", b"value").unwrap();
    assert_eq!(db.get(b"key").unwrap(), Some(b"value".to_vec()));
}

#[test]
fn put_overwrites_existing_value() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.put(b"key", b"old").unwrap();
    db.put(b"key", b"new").unwrap();
    assert_eq!(db.get(b"key").unwrap(), Some(b"new".to_vec()));
}

#[test]
fn delete_removes_key() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.put(b"key", b"value").unwrap();
    db.delete(b"key").unwrap();
    assert_eq!(db.get(b"key").unwrap(), None);
}

#[test]
fn delete_missing_key_is_ok() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.delete(b"missing").unwrap();
    assert_eq!(db.get(b"missing").unwrap(), None);
}

#[test]
fn iter_empty_database() {
    let db = Db::open("/tmp/rustone-test").unwrap();
    assert_eq!(db.iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn iter_returns_sorted_live_entries() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.put(b"c", b"3").unwrap();
    db.put(b"a", b"1").unwrap();
    db.put(b"b", b"2").unwrap();

    let entries: Vec<_> = db
        .iter()
        .map(|(key, value)| (key.to_vec(), value.to_vec()))
        .collect();

    assert_eq!(
        entries,
        vec![
            (b"a".to_vec(), b"1".to_vec()),
            (b"b".to_vec(), b"2".to_vec()),
            (b"c".to_vec(), b"3".to_vec()),
        ]
    );
}

#[test]
fn iter_skips_deleted_keys() {
    let mut db = Db::open("/tmp/rustone-test").unwrap();
    db.put(b"a", b"1").unwrap();
    db.put(b"b", b"2").unwrap();
    db.put(b"c", b"3").unwrap();
    db.delete(b"b").unwrap();

    let keys: Vec<_> = db.iter().map(|(key, _)| key.to_vec()).collect();
    assert_eq!(keys, vec![b"a".to_vec(), b"c".to_vec()]);
}
