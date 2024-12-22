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

pub struct Stats {  
    pub num_keys: u64,
    pub num_bytes: u64,
    pub num_locks: u64,
    pub num_pending_compactions: u64,
}

pub fn get_stats() -> Result<Stats, rocksdb::Error> {
    let db = get_db();
    Ok(Stats {
        num_keys: db.property_value("rocksdb.estimate-num-keys")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_bytes: db.property_value("rocksdb.estimate-live-data-size")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_locks: db.property_value("rocksdb.num-locks-active")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_pending_compactions: db.property_value("rocksdb.pending-compaction-bytes")?.and_then(|s| s.parse().ok()).unwrap_or(0),
    })
}   