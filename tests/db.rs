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
