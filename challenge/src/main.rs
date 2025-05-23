use challenge::node::Node;
use reqwest::blocking::{Client, Response};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

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

fn db_updater(db: Arc<Mutex<Connection>>) {
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
fn stream(listener: TcpListener, db: Arc<Mutex<Connection>>) {
    for stream in listener.incoming() {
        let db_lock = db.lock().expect("Could not lock db");
        let json = serde_json::to_string_pretty(&retrive_db(&db_lock))
            .expect("Failed to convert data to JSON");
        response(stream.expect("Stream could not be established"), json);
    }
}

fn response(mut stream: TcpStream, json: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read stream");

    println!("Received buffer: {:?}", String::from_utf8_lossy(&buffer));
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        json
    );
    println!("{} \n this is json response", json);

    stream
        .write_all(response.as_bytes())
        .expect("Could not respond to client");
    stream.flush().unwrap();
}

fn main() {
    let main_db = Arc::new(Mutex::new(create_db()));
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind the port");
    db_updater(Arc::clone(&main_db));
    stream(listener, main_db);
}
