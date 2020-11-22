extern crate drash;
use std::collections::HashMap;

// Create a resource
pub fn home_resource () -> drash::http::resource::Resource {
    fn get(_request: drash::http::request::Request, response: &drash::http::response::Response) -> &drash::http::response::Response {
        println!("GET HomeResource");
        return response;
    }
    let mut home_resource_methods = HashMap::new();
    home_resource_methods.insert(String::from("get"), get as fn(request: drash::http::request::Request, response: &drash::http::response::Response) -> &drash::http::response::Response);
    drash::http::resource::new(
        vec![
            String::from("/")
        ],
        home_resource_methods
    )
}