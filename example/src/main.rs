extern crate drash;

#[path = "resources.rs"]
mod resources;

//mod resources;
//use crate::resources;
//use std::io::prelude::*;
//use std::io::BufReader;
//use std::net::{TcpListener, TcpStream};
//use std::io::Write;
//use std::io::Read;
//use std::thread;
//use std::sync::mpsc;
//use std::sync::Arc;
//use std::sync::Mutex;

fn main () {

    // Create a resource
    let home_resource = resources::home_resource::home_resource();

    // Create server
    let server_configs = drash::http::server::Configs {
        resources: vec![
            home_resource
        ]
    };
    let server = drash::http::server::new(
        server_configs
    );

    // Run the server
    let options = &drash::http::server::HttpOptions {
        hostname: String::from("0.0.0.0"),
        port: 1334
    };
    server.run(options);

    // let listener = TcpListener::bind(String::from("0.0.0.0:1334")).unwrap();
    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     thread::spawn(move || {
    //         let mut buffer = [0; 1024];
    //         stream.read(&mut buffer).unwrap();
    //         println!("Start of buffer:");
    //         println!("{}", String::from_utf8_lossy(&buffer[..]));
    //         println!("End of buffer.");
    //         let contents = String::from("hello");
    //         let response = format!(
    //             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //             contents.len(),
    //             contents
    //         );
    //         // println!("start");
    //         // for header in BufReader::new(&mut stream).lines() {
    //         //     let header = header.unwrap();
    //         //     println!("{}", header);
    //         //     if header == "" {
    //         //         break
    //         //     }
    //         // }
    //         // println!("endd");
    //         stream.write_all(response.as_bytes()).unwrap();
    //         stream.flush().unwrap();
    //     });
    // }
}
