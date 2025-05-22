use reqwest::blocking::{Client, Response};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "publicKey")]
    pub_key: String,
    alias: String,
    capacity: u64,
    #[serde(rename = "firstSeen")]
    first_seen: i64,
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
        conn.execute(
            "INSERT INTO node (pubkey, alias, capacity, first_seen) VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT(pubkey) DO UPDATE SET capacity = EXCLUDED.capacity, first_seen = EXCLUDED.first_seen",
        (&node.pub_key, &node.alias, node.capacity, node.first_seen),
    )
    .expect("Could not insert into node table");
    }
}
fn retrive_db(conn: &Connection) -> Vec<Node> {
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
    println!("{:?}", retrive_db(&db));
}
