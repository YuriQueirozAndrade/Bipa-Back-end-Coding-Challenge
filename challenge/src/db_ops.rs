use crate::network::retrive_node;
use crate::node::{Cache, Node};
use rusqlite::Result;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum DbError {
    CreateError,
    InsertError,
    RetriveError,
    UpdateError,
}

pub fn create_db() -> Result<Connection, DbError> {
    match Connection::open("./nodes.db"){
        Ok(conn) => {
            match conn.execute(
                "CREATE TABLE IF NOT EXISTS node (pubkey TEXT PRIMARY KEY, alias TEXT, capacity INTEGER, first_seen INTEGER)",()
            ) {
                Ok(_) => Ok(conn),
                Err(_) => Err(DbError::CreateError),
            }
        }
        Err(_) => Err(DbError::CreateError),
    }
}
pub fn insert_db(conn: &mut Connection, nodes: Vec<Node>) -> Result<(), DbError> {
    let tx = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(_) => return Err(DbError::InsertError),
    };
    {
        let mut stmt = match  tx.prepare(
            "INSERT INTO node (pubkey, alias, capacity, first_seen) VALUES (?, ?, ?, ?)
            ON CONFLICT(pubkey) DO UPDATE SET capacity = EXCLUDED.capacity, first_seen = EXCLUDED.first_seen"
            ) {
            Ok(statement) => statement,
            Err(_) => return  Err(DbError::InsertError),
        };
        for node in &nodes {
            match stmt.execute(params![
                &node.pub_key,
                &node.alias,
                node.capacity,
                node.first_seen,
            ]) {
                Ok(_) => (),
                Err(_) => return Err(DbError::InsertError),
            }
        }
    }
    match tx.commit() {
        Ok(_) => Ok(()),
        Err(_) => Err(DbError::InsertError),
    }
}
pub fn retrive_db(conn: &Connection) -> Vec<Node> {
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
            let mut locked_db = db.lock().unwrap();
            let _ = insert_db(&mut locked_db, nodes);

            let mut cache_lock = node_cache.lock().unwrap();
            cache_lock.expired = true;
        }
        println!("Database Update");
        thread::sleep(Duration::from_secs(10));
    });
}
