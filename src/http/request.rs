use std::net::TcpStream;
use std::io::prelude::*;
// #[path = "response.rs"]
// mod response;
use crate::Http::Response;
use crate::Http::Server;
use std::net::Incoming;

pub struct Request<'a> {
    pub stream: &'a TcpStream
}
impl Request<'_> {

}