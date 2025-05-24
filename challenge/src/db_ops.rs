use crate::network::retrive_node;
use crate::node::{Cache, Node};
use crate::constants::{DB_PATH, TIME_UPDATE};
use rusqlite::{params, Connection, Result};
use std::{
    fmt,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub enum DbError {
    CreateError,
    InsertError,
    RetriveError,
    UpdateError,
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DbError::CreateError => "CreateError",
                DbError::InsertError => "InsertError",
                DbError::RetriveError => "RetriveError",
                DbError::UpdateError => "UpdateError",
            }
        )
    }
}

// future improvment: make the error less verbose

pub fn create_db() -> Result<Connection, DbError> {
    match Connection::open(DB_PATH){
        Ok(conn) => {
            match conn.execute(
                "CREATE TABLE IF NOT EXISTS node (pubkey TEXT PRIMARY KEY, alias TEXT, capacity INTEGER, first_seen INTEGER)",()
            ) {
                Ok(_) => Ok(conn),
                Err(e) => {
                    eprintln!("Error on create table: {}",e);
                    Err(DbError::CreateError)
                }
            }
        }
        Err(e) => {
            eprintln!("Error on connection: {}",e);
            Err(DbError::CreateError)
        }
    }
}
 fn insert_db(conn: &mut Connection, nodes: Vec<Node>) -> Result<(), DbError> {
    if nodes.is_empty(){
        return Ok(());
    }
    let tx = match conn.transaction() {
        Ok(transaction) => transaction,
        Err(e) => {
            eprintln!("Error on create transaction: {}", e);
            return Err(DbError::InsertError);
        }
    };
    {
        let mut stmt = match  tx.prepare(
            "INSERT INTO node (pubkey, alias, capacity, first_seen) VALUES (?, ?, ?, ?)
            ON CONFLICT(pubkey) DO UPDATE SET capacity = EXCLUDED.capacity, first_seen = EXCLUDED.first_seen"
            ) {
            Ok(statement) => statement,
            Err(e) => {
                eprintln!("Error on prepare statament: {}", e);
                return  Err(DbError::InsertError)
            } 
        };
        for node in &nodes {
            match stmt.execute(params![
                &node.pub_key,
                &node.alias,
                node.capacity,
                node.first_seen,
            ]) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error on execute statament: {}", e);
                    return Err(DbError::InsertError)
                } 
            }
        }
    }
    match tx.commit() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Error on commit transaction: {}",e);
            Err(DbError::InsertError)
        }
    }
}

pub fn retrive_db(conn: &Connection) -> Result<Vec<Node>, DbError> {
    let mut stmt = match conn.prepare("SELECT pubkey, alias, capacity, first_seen FROM node") {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("Error on execute statament: {}", e);
            return Err(DbError::RetriveError)
        } 
    };
    match stmt.query_map([], |row| {
        Ok(Node {
            pub_key: row.get(0)?,
            alias: row.get(1)?,
            capacity: row.get(2)?,
            first_seen: row.get(3)?,
        })
    }) {
        Ok(node_iter) => Ok(node_iter.filter_map(Result::ok).collect()),
        Err(e) => {
            eprintln!("Error on retribe nodes: {}", e);
            Err(DbError::RetriveError)
        } 
    }
}

pub fn db_updater(
    db: Arc<Mutex<Connection>>,
    node_cache: Arc<Mutex<Cache>>,
) -> Result<(), DbError> {
    thread::spawn(move || loop {
        {
            let mut locked_db = match db.lock() {
                Ok(locked_db) => locked_db,
                Err(e) => {
                    eprintln!("Error on lock database: {}", e);
                    return DbError::UpdateError
                } 
            };
            let mut cache_lock = match node_cache.lock() {
                Ok(cache) => cache,
                Err(e) => {
                    eprintln!("Error on lock cache nodes: {}", e);
                    return DbError::UpdateError
                } 
            };
            let nodes: Vec<Node> = match retrive_node().expect("").json() {
                Ok(nodes) => nodes,
                Err(e) => {
                    eprintln!("Error on call retrive_node: {}", e);
                    return DbError::UpdateError
                } 
            };
            match insert_db(&mut locked_db, nodes) {
                Ok(_) => cache_lock.expired = true,
                Err(e) => {
                    eprintln!("Error on call insert_db: {}", e);
                    return DbError::UpdateError
                } 
            };
        }
        println!("Database Update");
        thread::sleep(TIME_UPDATE);
    });
    Ok(())
}
