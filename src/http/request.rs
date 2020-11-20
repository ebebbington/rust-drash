use std::net::TcpStream;
use std::io::prelude::*;
// #[path = "response.rs"]
// mod response;
use chrono::prelude::*;
use crate::Http::Response;
use crate::Http::Server;
use std::net::Incoming;

pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub time: DateTime<Local>
}
impl Request {

}

pub fn new (request: &str) -> Request {
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // let request = String::from_utf8_lossy(&buffer[..]);
    let mut parts = request.split(" ");
    let method = String::from(parts.next().as_deref().unwrap_or(""));
    let path = String::from(parts.next().as_deref().unwrap_or(""));
    let http_version = String::from(parts.next().as_deref().unwrap_or(""));
    let time = Local::now();
    let Request = Request { method, path, http_version, time };
    Request
}