use crate::thread_ops::Cache;
use reqwest::blocking::{Client, Response};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn retrive_node() -> Response {
    Client::new()
        .get("https://mempool.space/api/v1/lightning/nodes/rankings/connectivity")
        .send()
        .expect("Could not get data from the endpoint")
}
pub fn listener(str_bind: &str) -> TcpListener {
    TcpListener::bind(str_bind).expect("Could not bind the port")
}
pub fn stream(listener: TcpListener, node: Arc<Mutex<Cache>>, db: Arc<Mutex<Connection>>) {
    for stream in listener.incoming() {
        let locked_db = db.lock().unwrap();
        let nodes_array = node.lock().unwrap().call_data(&locked_db);
        let json = serde_json::to_string_pretty(&nodes_array).unwrap();
        response(stream.expect("Stream could not be established"), json);
    }
}

pub fn response(mut stream: TcpStream, json: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read stream");

    println!("Received buffer: {:?}", String::from_utf8_lossy(&buffer));
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
        json
    );
    //println!("{} \n this is json response", json);

    stream
        .write_all(response.as_bytes())
        .expect("Could not respond to client");
    stream.flush().unwrap();
}
