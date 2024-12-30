use rocksdb::{OptimisticTransactionDB, Options, DB, WriteOptions, DBCompactionStyle};
use std::sync::Arc;
use parking_lot::RwLock;
use once_cell::sync::Lazy;

// Use RwLock instead of Mutex for better read concurrency
static DB_INSTANCE: Lazy<Arc<RwLock<OptimisticTransactionDB>>> = Lazy::new(|| {
    let path = "database";
    let mut opts = Options::default();
    opts.create_if_missing(true);
    
    // Optimize for high concurrent workloads
    opts.set_max_background_jobs(num_cpus::get() as i32 * 2);  // Double the background jobs
    opts.set_max_write_buffer_number(8);  // Increase write buffers further
    opts.set_min_write_buffer_number_to_merge(2);
    opts.set_level_zero_file_num_compaction_trigger(8);
    opts.increase_parallelism(num_cpus::get() as i32);
    opts.set_max_open_files(-1);  // Unlimited open files
    opts.set_keep_log_file_num(10);
    
    // Optimize write path
    opts.set_write_buffer_size(512 * 1024 * 1024);  // 512MB write buffer
    opts.set_max_total_wal_size(2 * 1024 * 1024 * 1024);  // 2GB WAL size
    opts.set_allow_concurrent_memtable_write(true);
    opts.set_enable_write_thread_adaptive_yield(true);
    
    // Additional write optimizations
    opts.set_use_direct_io_for_flush_and_compaction(true);  // Use direct I/O
    opts.set_bytes_per_sync(1048576);  // 1MB sync
    opts.set_compaction_style(DBCompactionStyle::Level);
    opts.optimize_for_point_lookup(1024);  // Optimize for point lookups with 1KB block cache
    opts.set_target_file_size_base(64 * 1024 * 1024);  // 64MB target file size
    
    let db = OptimisticTransactionDB::open(&opts, path).expect("Failed to open database connection");
    Arc::new(RwLock::new(db))
});

pub fn get_db() -> &'static Arc<RwLock<OptimisticTransactionDB>> {
    &DB_INSTANCE
}

fn open_readonly_db(path: &str) -> Result<DB, rocksdb::Error> {
    let mut opts = Options::default();
    opts.set_max_open_files(-1);
    opts.increase_parallelism(num_cpus::get() as i32);
    opts.set_use_direct_io_for_flush_and_compaction(true);
    DB::open_for_read_only(&opts, path, false)
}

pub fn insert_data(key: &str, value: &str) -> Result<(), rocksdb::Error> {
    let db = get_db();
    let mut write_opts = WriteOptions::default();
    write_opts.set_sync(false);  // Async writes for better performance
    write_opts.disable_wal(false);  // Keep WAL for durability
    write_opts.set_no_slowdown(true);  // Don't slow down on high write load
    
    db.read().put_opt(key.as_bytes(), value.as_bytes(), &write_opts)
}

pub fn get_data(key: &str) -> Result<Option<Vec<u8>>, rocksdb::Error> {
    let db = get_db();
    db.read().get(key.as_bytes())
}

pub struct Stats {  
    pub num_keys: u64,
    pub num_bytes: u64,
    pub num_locks: u64,
    pub num_pending_compactions: u64,
}

pub fn get_stats() -> Result<Stats, rocksdb::Error> {
    let db = open_readonly_db("database")?;
    Ok(Stats {
        num_keys: db.property_value("rocksdb.estimate-num-keys")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_bytes: db.property_value("rocksdb.estimate-live-data-size")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_locks: db.property_value("rocksdb.num-locks-active")?.and_then(|s| s.parse().ok()).unwrap_or(0),
        num_pending_compactions: db.property_value("rocksdb.pending-compaction-bytes")?.and_then(|s| s.parse().ok()).unwrap_or(0),
    })
}   