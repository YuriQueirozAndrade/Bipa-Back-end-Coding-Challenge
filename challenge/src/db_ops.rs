use crate::network::retrive_node;
use crate::node::{Cache, Node};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn create_db() -> Connection {
    let conn = Connection::open("./nodes.db").expect("Could not start a connection with data base");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS node ( pubkey TEXT PRIMARY KEY, alias TEXT, capacity INTEGER, first_seen INTEGER)",
        (),
    )
    .expect("Error on create the table on db");
    conn
}
pub fn insert_db(conn: &Connection, nodes: Vec<Node>) {
    for node in &nodes {
        // Future improvment: try put all in one sql query,dont use the for loop
        conn.execute(
            "INSERT INTO node (pubkey, alias, capacity, first_seen) VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(pubkey) DO UPDATE SET capacity = EXCLUDED.capacity, first_seen = EXCLUDED.first_seen",
        (&node.pub_key, &node.alias, node.capacity, node.first_seen),
    )
    .expect("Could not insert into node table");
    }
}
pub fn retrive_db(conn: &Connection) -> Vec<Node> {
    // Future improvment:keep the json file in memorie, if data base donts update, send the in
    // memmori json, if update, make a new query
    let mut stmt = conn
        .prepare("SELECT pubkey, alias, capacity, first_seen FROM node")
        .expect("Error on prepare the sql query");
    let node_inter = stmt
        .query_map([], |row| {
            Ok(Node {
                pub_key: row.get(0).expect("Could not get pub_key"),
                alias: row.get(1).expect("Could not get alias"),
                capacity: row.get(2).expect("Could not get capacity"),
                first_seen: row.get(3).expect("Could not get first_seen"),
            })
        })
        .expect("Not is possible make the query for retrive all data from db");
    let nodes: Vec<Node> = node_inter.filter_map(Result::ok).collect();
    nodes
}

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
