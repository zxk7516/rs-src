use std::collections::HashMap;

pub struct Request {
    pub path_and_query: String,
    pub headers: HashMap<String,String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub struct Response {
    pub status u16,
    pub headers: HashMap<String,String>,
    pub body: Vec<u8>,
}
