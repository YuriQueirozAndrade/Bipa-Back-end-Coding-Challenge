use crate::constants::{END_POINT_CHALLENGE, RETRIVE_NODES_URL};
use crate::node::Cache;
use reqwest::blocking::{Client, Response};
use rusqlite::Connection;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub enum NetworkError {
    RetriveError,
    StreamError,
    ResponseError,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::RetriveError => write!(f, "Retrieval error occurred"),
            NetworkError::StreamError => write!(f, "Stream error occurred"),
            NetworkError::ResponseError => write!(f, "Response error occurred"),
        }
    }
}

pub fn retrive_node() -> Result<Response, NetworkError> {
    match Client::new().get(RETRIVE_NODES_URL).send() {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("Error on retrive data from node: {}", e);
            Err(NetworkError::RetriveError)
        }
    }
}
pub fn listener(str_bind: &str) -> TcpListener {
    TcpListener::bind(str_bind).expect("Could not bind the port")
}

pub fn stream(
    listener: TcpListener,
    node: Arc<Mutex<Cache>>,
    db: Arc<Mutex<Connection>>,
) -> Result<(), NetworkError> {
    for stream in listener.incoming() {
        let locked_db = match db.lock() {
            Ok(locked_db) => locked_db,
            Err(e) => {
                eprintln!("Error on lock database: {}", e);
                continue;
            }
        };

        let nodes_array = match node.lock() {
            Ok(mut nodes_array) => nodes_array.call_data(&locked_db),
            Err(e) => {
                eprintln!("Error on acess node data: {}", e);
                continue;
            }
        };

        let json = match serde_json::to_string_pretty(&nodes_array) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error on serilize node to json: {}", e);
                continue;
            }
        };

        match stream {
            Ok(stream) => {
                if let Err(e) = response(stream, json) {
                    eprintln!("Error on respond stream: {}", e);
                }
            }
            Err(e) => eprintln!("Error on establish stream: {}", e),
        }
    }

    Ok(())
}

pub fn response(mut stream: TcpStream, json: String) -> Result<(), NetworkError> {
    let mut buffer = [0; 1024];

    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read stream: {}", e);
        return Err(NetworkError::ResponseError);
    }

    let request = String::from_utf8_lossy(&buffer);
    let response = if request.starts_with(END_POINT_CHALLENGE) {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
            json
        )
    } else {
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nEndpoint not found".to_string()
    };

    match stream.write_all(response.as_bytes()) {
        Ok(_) => {
            let _ = stream.flush();
            Ok(())
        }
        Err(e) => {
            eprintln!("Error on write response: {}", e);
            Err(NetworkError::ResponseError)
        }
    }
}
