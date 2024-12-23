use rocksdb::{TransactionDB, Options, DB};
use std::sync::Mutex;
use once_cell::sync::Lazy;
static DB_INSTANCE: Lazy<Mutex<TransactionDB>> = Lazy::new(|| {
    let path = "database";
    let db = TransactionDB::open_default(path).expect("Failed to open database connection");
    Mutex::new(db)
});

fn open_readonly_db(path: &str) -> Result<DB, rocksdb::Error> {
    let mut opts = Options::default();
    opts.set_max_open_files(-1);  // or any other needed config
    DB::open_for_read_only(&opts, path, /*error_if_log_file_exists=*/false)
}


pub fn get_db() -> &'static Mutex<TransactionDB> {
    &DB_INSTANCE
}

pub fn insert_data(key: &str, value: &str) -> Result<(), rocksdb::Error> {
    let db = get_db();
    db.lock().unwrap().put(key.as_bytes(), value.as_bytes())
}

pub fn get_data(key: &str) -> Result<Option<Vec<u8>>, rocksdb::Error> {
    let db = get_db();  
    db.lock().unwrap().get(key.as_bytes())
}   

pub struct Stats {  
    pub num_keys: u64,
    pub num_bytes: u64,
    pub num_locks: u64,
    pub num_pending_compactions: u64,
}

pub fn get_stats() -> Result<Stats, rocksdb::Error> {
    let db = open_readonly_db("database").expect("Failed to open database");
    Ok(Stats {
        num_keys: db.property_value("rocksdb.estimate-num-keys")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_bytes: db.property_value("rocksdb.estimate-live-data-size")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_locks: db.property_value("rocksdb.num-locks-active")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_pending_compactions: db.property_value("rocksdb.pending-compaction-bytes")?.and_then(|s| s.parse().ok()).unwrap_or(0),
    })
}   