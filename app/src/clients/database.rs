use rocksdb::{DB, Options};

pub fn get_db() -> DB {
    let path = "database";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, path).expect("Failed to open database connection")
}

pub fn insert_data(key: &str, value: &str) -> Result<(), rocksdb::Error> {
    let db = get_db();
    db.put(key.as_bytes(), value.as_bytes())
}

pub fn get_data(key: &str) -> Result<Option<Vec<u8>>, rocksdb::Error> {
    let db = get_db();  
    db.get(key.as_bytes())
}   