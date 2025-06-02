use crate::db_ops::{retrive_db, retrive_db_order_by};
use chrono::{TimeZone, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize, Serializer};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    pub pub_key: String,
    pub alias: String,

    #[serde(rename = "capacity")]
    #[serde(serialize_with = "s_capacity")]
    pub capacity: u64,

    #[serde(rename = "firstSeen")]
    #[serde(serialize_with = "s_timestamp")]
    pub first_seen: i64,
}
#[derive(Default)]
pub struct Cache {
    pub expired: bool,
    pub nodes: Vec<Node>,
}

impl Cache {
    pub fn call_data(&mut self, db: &Connection) -> Vec<Node> {
        if true {
            println!("Cache exipired make a new request from db");
            self.nodes = match retrive_db(db) {
                Ok(nodes) => nodes,
                Err(e) => {
                    eprintln!("Error on cache, fail to retrive_db: {}", e);
                    Vec::new()
                }
            };
            self.expired = false;
        }
        self.nodes.clone()
    }
    pub fn call_data_oderder_by(&mut self, db: &Connection) -> Vec<Node> {
        if true {
            println!("Cache exipired make a new request from db");
            self.nodes = match retrive_db_order_by(db) {
                Ok(nodes) => nodes,
                Err(e) => {
                    eprintln!("Error on cache, fail to retrive_db: {}", e);
                    Vec::new()
                }
            };
            self.expired = false;
        }
        self.nodes.clone()
    }
    pub fn new() -> Cache {
        Cache {
            expired: false,
            nodes: Vec::new(),
        }
    }
}

fn s_capacity<S>(capacity: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_f64(*capacity as f64 / 100_000_000.0)
}

fn s_timestamp<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match Utc.timestamp_opt(*timestamp, 0).single() {
        Some(datetime) => serializer.serialize_str(&datetime.to_rfc3339()),
        None => Err(serde::ser::Error::custom("Invalid timestamp")),
    }
}
