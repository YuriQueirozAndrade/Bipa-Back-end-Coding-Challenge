use crate::db_ops::insert_db;
use crate::network::retrive_node;
use crate::node::Node;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn db_updater(db: Arc<Mutex<Connection>>) {
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
