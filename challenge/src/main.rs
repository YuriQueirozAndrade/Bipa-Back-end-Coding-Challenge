use challenge::db_ops::{create_db, insert_db, retrive_db};
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
