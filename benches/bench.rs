use rustone::Db;

fn main() {
    let mut db = Db::open("/tmp/rustone-bench").expect("open db");

    for i in 0..1_000u64 {
        let key = format!("key-{i}");
        db.put(key.as_bytes(), b"value").expect("put");
    }
}
