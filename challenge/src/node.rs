use crate::db_ops::retrive_db;
use chrono::{TimeZone, Utc};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
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
        if self.expired {
            print!("Cache exipired make a new request from db");
            self.nodes = retrive_db(db);
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
    S: serde::Serializer,
{
    serializer.serialize_f64(*capacity as f64 / 100_000_000.0)
}

fn s_timestamp<S>(timestamp: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let datetime = Utc.timestamp_opt(*timestamp, 0).unwrap();
    serializer.serialize_str(&datetime.to_rfc3339())
}
