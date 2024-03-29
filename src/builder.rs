use std::collections::HashMap;

use crate::errors::{BuilderError, Error};
use crate::request::Request;

/// Describes the request builder
pub trait ValidBuilder {
    /// Create a new builder
    fn new() -> Self;

    /// Build the request from the builder
    fn build(&self) -> Result<Request, BuilderError>;

    /// Set the host of the request (example: httpbin.org)
    fn host(&mut self, host: String) -> &mut Self;

    /// Set the port of the request (example: 80)
    fn port(&mut self, port: u16) -> &mut Self;

    /// Set the HTTP version of the request (example: 1.1)
    fn http_version(&mut self, http_version: String) -> &mut Self;

    /// Set the method of the request (example: GET)
    fn method(&mut self, method: String) -> &mut Self;

    /// Set the path of the request (example: /post)
    fn path(&mut self, path: String) -> &mut Self;

    /// Set a header of the request (example: Content-Type, application/json)
    fn header(&mut self, key: String, value: String) -> &mut Self;

    /// Set the body of the request (example: {"name": "John"})
    fn body(&mut self, body: String) -> &mut Self;

    /// Set the URI of the request (example: http://httpbin.org/post?name=bob)
    fn uri(&mut self, uri: String) -> &mut Self;
}

/// The request builder
#[derive(Clone, Debug)]
pub struct Builder {
    /// The host of the request (example: `httpbin.org`)
    pub host: Option<String>,

    /// The port of the request (example: `80`)
    pub port: Option<u16>,

    /// The HTTP version of the request (example: `1.1`)
    pub http_version: Option<String>,

    /// The method of the request (example: `GET`)
    pub method: Option<String>,

    /// The path of the request (example: `/post`)
    pub path: Option<String>,

    /// The headers of the request (example: `Content-Type: application/json`)
    pub headers: HashMap<String, String>,

    /// The body of the request (example: `{"name": "John"}`)
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
            method: Some("GET".to_string()),
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
