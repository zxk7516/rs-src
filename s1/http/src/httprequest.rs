use std::{
    collections::HashMap,
    io::BufRead,
};

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_msg_body = String::new();
        let mut parsed_headers = HashMap::new();
        for line in req.lines() {
            if line.starts_with("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {
            } else {
                parsed_msg_body = line.to_string();
            }
        }
        HttpRequest {
            version: parsed_version,
            headers: parsed_headers,
            method: parsed_method,
            msg_body: parsed_msg_body,
            resource: parsed_resource,
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.into()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let mut header_item = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_item.next() {
        key = k.to_string();
    }
    if let Some(v) = header_item.next() {
        value = v.to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);

        let m: Method = "POST".into();
        assert_eq!(m, Method::Post);
    }
    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);

        let m: Method = "POST".into();
        assert_eq!(m, Method::Post);
    }

    #[test]
    fn test_read_http() {
        let s: String = String::from(
            "GET /greeting HTTP/1.1\r\nHost: localhost\r\nAccept:*/*\r\nUserAgent:curl/7.71.1",
        );
        let req: HttpRequest = s.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".into()), req.resource);
    }
}
