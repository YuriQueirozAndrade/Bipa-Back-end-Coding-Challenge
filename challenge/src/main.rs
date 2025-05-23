use challenge::db_ops::create_db;
use challenge::network::{listener, stream};
use challenge::thread_ops::db_updater;
use std::sync::{Arc, Mutex};

fn main() {
    let main_db = Arc::new(Mutex::new(create_db()));
    let listener = listener("127.0.0.1:8080");
    db_updater(Arc::clone(&main_db));
    stream(listener, main_db);
}
