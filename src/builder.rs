use std::collections::HashMap;

use crate::errors::{BuilderError, Error};
use crate::request::Request;

pub trait ValidBuilder {
    fn new() -> Self;
    fn build(&self) -> Result<Request, BuilderError>;
    fn host(&mut self, host: String) -> &mut Self;
    fn port(&mut self, port: u16) -> &mut Self;
    fn http_version(&mut self, http_version: String) -> &mut Self;
    fn method(&mut self, method: String) -> &mut Self;
    fn path(&mut self, path: String) -> &mut Self;
    fn header(&mut self, key: String, value: String) -> &mut Self;
    fn body(&mut self, body: String) -> &mut Self;
    fn uri(&mut self, uri: String) -> &mut Self;
}

#[derive(Clone, Debug)]
pub struct Builder {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub http_version: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl ValidBuilder for Builder {
    fn new() -> Builder {
        let mut headers = HashMap::new();
        headers.insert("Connection".to_string(), "close".to_string());
        Builder {
            host: None,
            port: Some(80),
            http_version: Some("1.1".to_string()),
            method: None,
            path: None,
            headers: headers,
            body: None,
        }
    }

    fn build(&self) -> Result<Request, BuilderError> {
        if self.host.is_none() {
            return Err(BuilderError::new("host is required".to_string()));
        }
        if self.method.is_none() {
            return Err(BuilderError::new("method is required".to_string()));
        }
        if self.path.is_none() {
            return Err(BuilderError::new("path is required".to_string()));
        }
        if self.headers.is_empty() {
            return Err(BuilderError::new("headers are required".to_string()));
        }
        Ok(Request {
            host: self.host.clone().unwrap(),
            port: self.port.unwrap(),
            http_version: self.http_version.clone().unwrap(),
            method: self.method.clone().unwrap(),
            path: self.path.clone().unwrap(),
            headers: self.headers.clone(),
            body: self.body.clone().unwrap_or("".to_string()),
        })
    }

    fn host(&mut self, host: String) -> &mut Self {
        self.host = Some(host);
        self
    }

    fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    fn http_version(&mut self, http_version: String) -> &mut Self {
        self.http_version = Some(http_version);
        self
    }

    fn method(&mut self, method: String) -> &mut Self {
        self.method = Some(method);
        self
    }

    fn path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    fn header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    fn body(&mut self, body: String) -> &mut Self {
        self.body = Some(body);
        self
    }

    // valid uri formats:
    // http://example.com
    // http://example.com/
    // http://example.com/path
    // http://example.com/path:80
    fn uri(&mut self, uri: String) -> &mut Self {
        let uri = uri.trim_start_matches("http://").to_string();
        let parts: Vec<&str> = uri.splitn(2, '/').collect();

        if parts.len() == 1 {
            let host = parts[0].to_string();
            self.host(host);
            self.path("/".to_string());
            return self;
        }

        let host = parts[0].to_string();
        self.host(host);
        if parts.len() > 1 {
            let path = parts[1].to_string();

            if parts.contains(&":") {
                let path_parts: Vec<&str> = path.splitn(2, ':').collect();
                let path = path_parts[0].to_string();
                let port = path_parts[1].parse::<u16>().unwrap();
                self.port(port);

                if path.starts_with("/") {
                    self.path(path);
                } else {
                    self.path("/".to_string() + &path);
                }

                return self;
            } else {
                if path.starts_with("/") {
                    self.path(path);
                } else {
                    self.path("/".to_string() + &path);
                }
            }
        }
        self
    }
}
