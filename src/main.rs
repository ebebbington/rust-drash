#[path = "http.rs"]
mod Http;
use std::collections::HashMap;
use std::net::TcpListener;
use std::net::Incoming;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use bufstream::BufStream;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn main () {

    fn GET (request: Http::Request::Request, response: Http::Response::Response) -> Http::Response::Response {
        return response;
    }
    let mut home_resource_methods: Http::Resource::ResourceMethods = HashMap::new();
    home_resource_methods.insert(String::from("GET"), GET);
    let home_resource = Http::Resource::Resource {
        paths: vec![
            String::from("/")
        ],
        methods: home_resource_methods
    };

    let options = &Http::Server::HttpOptions {
        hostname: String::from("0.0.0.0"),
        port: 1334
    };
    let server = Http::Server::Server {
        resources: vec![
            home_resource
        ],
        options
    };

    server.run();

    // let listener = TcpListener::bind(String::from("0.0.0.0:1334")).unwrap();
    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     let mut buffer = [0; 1024];
    //     stream.read(&mut buffer).unwrap();
    //     let contents = String::from("hello");
    //     let response = format!(
    //         "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //         contents.len(),
    //         contents
    //     );
    //     stream.write_all(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    // let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    // let pool = ThreadPool::new(4);
    //
    // for stream in listener.incoming() {
    //     pool.execute(move || {
    //         let mut stream = stream.unwrap();
    //         let mut buffer = [0; 1024];
    //         &stream.read(&mut buffer).unwrap();
    //
    //         let contents = String::from("hello");
    //         let response = format!(
    //             "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
    //             contents.len(),
    //             contents
    //         );
    //         stream.write_all(response.as_bytes()).unwrap();
    //         stream.flush().unwrap();
    //     });
    // }

    // let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    //
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     //thread::spawn(|| {
    //         handle_client(stream);
    //     //});
    // }
}

// fn handle_client(mut stream: TcpStream) {
//     // Read all the headers
//     for header in BufReader::new(&mut stream).lines() {
//         println!("{:?}", header);
//         let header = header.unwrap();
//         if header == "\r" { break }
//         if header == "" { break }
//     }
//
//     // Write our response
//     stream.write_all(b"HTTP/1.0 200 OK\r\n\r\nhello").unwrap();
//     stream.flush().unwrap()
// }