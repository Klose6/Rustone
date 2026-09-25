use rustone::Db;

#[test]
fn opens_a_database() {
    let db = Db::open("/tmp/rustone-integration-test").unwrap();
    assert!(db.get(b"key").unwrap().is_none());
}
