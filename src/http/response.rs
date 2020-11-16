use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

pub struct Response;
impl Response {
    pub fn write_response (mut stream: &TcpStream, true_response: Response) {
        //let contents = fs::read_to_string("index.html").unwrap();
        let contents = String::from("hello");

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}