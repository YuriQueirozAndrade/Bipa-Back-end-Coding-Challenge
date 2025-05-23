use challenge::db_ops::{create_db, insert_db};
use challenge::network::{retrive_node, stream};
use challenge::node::Node;
use rusqlite::Connection;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn db_updater(db: Arc<Mutex<Connection>>) {
    thread::spawn(move || loop {
        {
            let nodes: Vec<Node> = retrive_node().json().expect("Failed to parse JSON");
            let locked_db = db.lock().unwrap();
            insert_db(&locked_db, nodes);
        }
        println!("Execute Update of Data Base");
        thread::sleep(Duration::from_secs(10));
    });
}

fn main() {
    let main_db = Arc::new(Mutex::new(create_db()));
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind the port");
    db_updater(Arc::clone(&main_db));
    stream(listener, main_db);
}
