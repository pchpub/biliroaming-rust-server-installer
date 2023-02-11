// simple http server
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
};

use super::types::ErrorCode;

pub fn simple_http_server(root: &str) -> Result<(), Box<dyn std::error::Error>> {
    // root: /opt/BiliRoaming-Rust-Server/web
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream?;
        if handle_connection(stream, root).unwrap_or(true) {
            break;
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, root: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().map_or(Err(ErrorCode::Empty), |value| Ok(value))??;

    let line_1: Vec<&str> = request_line.rsplit(' ').collect();
    let path = line_1.get(1).map_or(Err(ErrorCode::Empty), |value| Ok(*value))?;

    if path.starts_with("/.well-known/acme-challenge") {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string(format!("{root}{path}"))?;

        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes())?;
        Ok(false)
    }else if path.starts_with("/signal/stop") {
        Ok(true)
    }else{
        Ok(false)
    }
    
}