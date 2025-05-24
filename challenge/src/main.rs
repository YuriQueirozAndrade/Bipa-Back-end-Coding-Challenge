use challenge::constants::{BIND, IP};
use challenge::db_ops::{create_db, db_updater};
use challenge::network::{listener, stream};
use challenge::node::Cache;
use std::sync::{Arc, Mutex};

fn main() {
    let main_db = Arc::new(Mutex::new(create_db().expect("Could not create db")));
    let start_cache = Arc::new(Mutex::new(Cache::new()));

    let _ = db_updater(Arc::clone(&main_db), Arc::clone(&start_cache));
    let address = format!("{}:{}", IP, BIND);
    let _ = stream(listener(&address), start_cache, main_db);
}
