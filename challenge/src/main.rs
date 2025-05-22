use chrono::{TimeZone, Utc};
use reqwest::blocking::{Client, Response};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    pub_key: String,
    alias: String,

    #[serde(rename = "capacity")]
    #[serde(serialize_with = "s_capacity")]
    capacity: u64,

    #[serde(rename = "firstSeen")]
    #[serde(serialize_with = "s_timestamp")]
    first_seen: i64,
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

fn retrive_node() -> Response {
    Client::new()
        .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
        .send()
        .expect("Could not get data from the endpoint")
}
fn create_db() -> Connection {
    let conn = Connection::open("./nodes.db").expect("Could not start a connection with data base");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS node ( pubkey TEXT PRIMARY KEY, alias TEXT, capacity INTEGER, first_seen INTEGER)",
        (),
    )
    .expect("Error on create the table on db");
    conn
}
fn insert_db(conn: &Connection, nodes: Vec<Node>) {
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
fn retrive_db(conn: &Connection) -> Vec<Node> {
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

fn main() {
    let nodes: Vec<Node> = retrive_node().json().expect("Failed to parse JSON");
    let db = create_db();
    insert_db(&db, nodes);
    let json =
        serde_json::to_string_pretty(&retrive_db(&db)).expect("Failed to convert data to JSON");
    println!("{}", json);
}
