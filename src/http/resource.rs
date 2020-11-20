use crate::Http;
use std::fmt;
use std::collections::HashMap;

type COUNT = u32;

//pub type ResourceMethods =  HashMap<String, fn(Http::Request::Request, &Http::Response::Response) -> &Http::Response::Response>;
pub type ResourceMethods = [(String, fn(Http::Request::Request, &Http::Response::Response) -> &Http::Response::Response)];

pub struct Resource {
    pub paths: Vec<String>,
    pub methods: OneResourceMethod
}
impl Resource {

}

pub fn new (paths: Vec<String>, methods: ResourceMethods) -> Resource {
    Resource { paths, methods }
}
// impl fmt::Display for Resource {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "({}, {})", self.paths, self.methods)
//     }
// }

// #[derive(Debug)]
// pub struct Resource;
// impl Resource {
//     pub fn doSomething (&self) {
//         println!("i did something!")
//   }
// }