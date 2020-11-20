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

pub struct Server {
    pub resources: Vec<Resource::Resource>,
    //pub options: &'a HttpOptions
}

impl Server {

    pub fn run(&self, options: &HttpOptions) {
        let port_str: &String = &options.port.to_string();
        let address = format!("{}:{}", options.hostname, port_str);
        let listener = TcpListener::bind(address).unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
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
        let request = String::from_utf8_lossy(&buffer[..]);
        let Request = Http::Request::new(&request);
        'outer: for resource in &self.resources {
            for path in &resource.paths {
                if path == &Request.path {
                //let formatted = format!(r"{} HTTP/1.1", path);
                //let regex = Regex::new(formatted.as_str()).unwrap();
                //if regex.is_match(&request_data) {
                    for (method, func) in &resource.methods {
                        //let regex_two = Regex::new(format!(r"{} {} HTTP/1.1", method, path).as_str()).unwrap();
                        //if regex_two.is_match(&request_data) {
                        if method == &Request.method {
                            // let request = Request::Request {
                            //     stream: &stream
                            // };
                            let mut Response = Http::Response::new(stream);
                            let true_response = func(Request, &Response);
                            &mut Response.write_response(&true_response);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
}

pub fn new (resources: Vec<Http::Resource::Resource>) -> Server {
    Server { resources }
}