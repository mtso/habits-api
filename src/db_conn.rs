use rocksdb::DB;
use std::sync::{Arc, Mutex};

/// Connection type of rocksdb::DB.
pub type DbConn = Arc<Mutex<DB>>;
