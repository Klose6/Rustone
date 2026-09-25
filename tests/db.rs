use rustone::Db;

#[test]
fn opens_a_database() {
    let db = Db::open("/tmp/rustone-integration-test").unwrap();
    assert!(db.get(b"key").unwrap().is_none());
}

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
