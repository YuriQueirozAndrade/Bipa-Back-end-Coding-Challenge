use challenge::constants::{BIND, IP};
use challenge::db_ops::{create_db, db_updater};
use challenge::network::{listener, stream};
use challenge::node::Cache;
use std::sync::{Arc, Mutex};

// improve error handler for recoveable
fn main() {
    let main_db = Arc::new(Mutex::new(create_db().unwrap()));
    let start_cache = Arc::new(Mutex::new(Cache::new()));

    let _ = db_updater(Arc::clone(&main_db), Arc::clone(&start_cache));
    let address = format!("{}:{}", IP, BIND);
    let listener = match listener(&address) {
        Ok(listener) => listener,
        Err(e) => {
            eprint!("Error on call listener:{}", e);
            return;
        }
    };
    let _ = stream(listener, start_cache, main_db);
}
