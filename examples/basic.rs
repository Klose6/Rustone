use rustone::Db;

fn main() -> rustone::Result<()> {
    let mut db = Db::open("/tmp/rustone-basic-example")?;

    db.put(b"hello", b"world")?;
    match db.get(b"hello")? {
        Some(value) => println!("hello => {}", String::from_utf8_lossy(&value)),
        None => println!("key not found"),
    }

    Ok(())
}
