use challenge::db_ops::{create_db, db_updater};
use challenge::network::{listener, stream};
use challenge::node::Cache;
use std::sync::{Arc, Mutex};

fn main() {
    let main_db = Arc::new(Mutex::new(create_db()));
    let start_cache = Arc::new(Mutex::new(Cache::new()));

    db_updater(Arc::clone(&main_db), Arc::clone(&start_cache));
    stream(listener("127.0.0.1:8080"), start_cache, main_db);
}
