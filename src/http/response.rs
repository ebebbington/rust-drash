use std::net::TcpStream;
use std::io::prelude::*;
//use std::fs;

pub struct Response {
    pub stream: TcpStream,
    pub body: String
}
impl Response {
    pub fn write_response (&mut self) {
        //let contents = fs::read_to_string("index.html").unwrap();
        let contents = String::from(&self.body);

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        self.stream.write_all(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}

pub fn new (stream: TcpStream) -> Response {
    let response = Response { stream, body: String::from("") };
    response
}