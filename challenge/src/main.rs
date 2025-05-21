use reqwest::blocking::{Client, Response};
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

fn main() {
    let nodes: Vec<Node> = retrive_node().json().expect("Failed to parse JSON");

    println!("{:?}", nodes);
}
