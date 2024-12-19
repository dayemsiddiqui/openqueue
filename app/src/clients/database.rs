use rocksdb::{DB, Options};

pub fn get_db() -> DB {
    let path = "database";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, path).expect("Failed to open database connection")
}

pub fn insert_data(db: &DB, key: &[u8], value: &[u8]) -> Result<(), rocksdb::Error> {
    db.put(key, value)
}

pub fn get_data(db: &DB, key: &[u8]) -> Result<Option<Vec<u8>>, rocksdb::Error> {
    db.get(key)
}   