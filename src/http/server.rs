use std::net::TcpListener;
use std::net::Incoming;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use crate::Http::Request;
use crate::Http::Resource;
use crate::Http::Response;
use std::stringify;
use regex::Regex;
use std::result::Result;
use crate::Http;

pub struct HttpOptions {
  pub hostname: String,
  pub port: u32
}

pub struct Server<'a> {
    pub resources: Vec<Resource::Resource>,
    pub options: &'a HttpOptions
}

impl Server<'_> {

    pub fn run(&self) {
        let port_str: &String = &self.options.port.to_string();
        let address = format!("{}:{}", &self.options.hostname, port_str);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_http_request(stream)
            //Response::write_response(&stream); //single threaded
            // thread::spawn(|| {  // multi threaded
            //     //&self.handle_request(stream);
            //     Response::write_response(&stream)
            // });
        }
    }

    fn handle_http_request (&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request_data = String::from_utf8_lossy(&buffer[..]);
        'outer: for resource in &self.resources {
            for path in &resource.paths {
                let formatted = format!(r"{} HTTP/1.1", path);
                let regex = Regex::new(formatted.as_str()).unwrap();
                if regex.is_match(&request_data) {
                    for (method, func) in &resource.methods {
                        let regex_two = Regex::new(format!(r"{} {} HTTP/1.1", method, path).as_str()).unwrap();
                        if regex_two.is_match(&request_data) {
                            let request = Request::Request {
                                stream: &stream
                            };
                            let response = Http::Response::Response;
                            let true_response = func(request, response);
                            Http::Response::Response::write_response(&stream, true_response);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}