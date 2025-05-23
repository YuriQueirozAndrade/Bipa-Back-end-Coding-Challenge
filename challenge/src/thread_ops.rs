use crate::db_ops::{insert_db, retrive_db};
use crate::network::retrive_node;
use crate::node::Node;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn db_updater(db: Arc<Mutex<Connection>>, node_cache: Arc<Mutex<Cache>>) {
    thread::spawn(move || loop {
        {
            let nodes: Vec<Node> = retrive_node().json().expect("Failed to parse JSON");
            let locked_db = db.lock().unwrap();
            insert_db(&locked_db, nodes);

            let mut cache_lock = node_cache.lock().unwrap();
            cache_lock.expired = true;
        }
        println!("Database Update");
        thread::sleep(Duration::from_secs(10));
    });
}

pub struct Cache {
    pub expired: bool,
    pub nodes: Vec<Node>,
}

impl Cache {
    pub fn call_data(&mut self, db: &Connection) -> Vec<Node> {
        if self.expired {
            print!("Cache exipired make a new request from db");
            self.nodes = retrive_db(db);
            self.expired = false;
        }
        self.nodes.clone()
    }
}
