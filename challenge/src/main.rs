use challenge::db_ops::create_db;
use challenge::network::{listener, stream};
use challenge::thread_ops::Cache;
use challenge::thread_ops::db_updater;
use std::sync::{Arc, Mutex};

fn main() {
    let main_db = Arc::new(Mutex::new(create_db()));
    let start_cache = Arc::new(Mutex::new(Cache {
        expired: true,
        nodes: Vec::new(),
    }));

    db_updater(Arc::clone(&main_db), Arc::clone(&start_cache));
    stream(listener("127.0.0.1:8080"), start_cache, main_db);
}
