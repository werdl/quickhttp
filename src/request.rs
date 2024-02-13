use std::collections::HashMap;
use std::future::Future;
use std::{io::{Read, Write}, net::TcpStream};
use crate::StatusCode;
use crate::{errors::{Error, RequestError}, response::Response};

/// Describes a valid request
pub trait ValidRequest {
    /// Create a new request
    fn new(
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: String,
        host: String,
        port: u16,
        http_version: String,
    ) -> Self;
    /// Send the request synchronously
    fn send(&self) -> Result<Response, RequestError>;
}

/// Trait to convert headers to a string
trait HeadersToString {
    fn headers_to_string(&self) -> String;
}

impl HeadersToString for HashMap<String, String> {
    fn headers_to_string(&self) -> String {
        let mut headers = String::new();
        for (key, value) in self.iter() {
            headers.push_str(&format!("{}: {}\r\n", key, value));
        }
        headers
    }
}

/// The request type
#[derive(Clone, Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub host: String,
    pub port: u16,
    pub http_version: String,
}

impl Request {
    fn fail(&self, message: &str) -> RequestError {
        RequestError::new(format!("RequestError: {}", message))
    }

    /// Send the request asynchronously, returning a future
    pub fn async_send(&self) -> impl Future<Output = Result<Response, RequestError>> + '_ {
        async move {
            self.send()
        }
    }
}

/// Implement the ValidRequest trait for the Request type
impl ValidRequest for Request {
    fn new(
        method: String,
        path: String,
        headers: HashMap<String, String>,
        body: String,
        host: String,
        port: u16,
        http_version: String,
    ) -> Request {
        Request {
            method,
            path,
            headers,
            body,
            host,
            port,
            http_version,
        }
    }

    fn send(&self) -> Result<Response, RequestError> {
        // no propagation of errors, just return a new error using self.fail and the match statement

        let stream = TcpStream::connect(format!("{}:{}", self.host, self.port));
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => return Err(self.fail("could not connect to server")),
        };

        let mut headers = self.headers.clone();

        if !headers.contains_key("Content-Length") {
            let content_length = self.body.len();
            headers.insert("Content-Length".to_string(), content_length.to_string());
        }

        let request = format!(
            "{} {} HTTP/{}\r\nHost: {}\r\n{}\r\n{}",
            self.method, self.path, self.http_version, self.host, headers.headers_to_string(), self.body
        );

        let _ = match stream.write(request.as_bytes()) {
            Ok(_) => (),
            Err(_) => return Err(self.fail("could not write request")),
        };

        let mut response = Vec::new();

        let _ = match stream.read_to_end(&mut response) {
            Ok(_) => (),
            Err(_) => return Err(self.fail("could not read response")),
        };

        // parse the response
        let response = String::from_utf8(response).unwrap();

        let mut parts = response.split("\r\n\r\n");
        let headers = parts.next().unwrap();
        let body = parts.next().unwrap();

        let mut headers = headers.split("\r\n");
        let status_line = headers.next().unwrap();
        let status_code = status_line.split(" ").nth(1).unwrap().parse::<u16>().unwrap();

        let headers = headers.map(|header| {
            let mut parts = header.split(": ");
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            (key.to_string(), value.to_string())
        });

        let mut headers_map = HashMap::new();

        for (key, value) in headers {
            headers_map.insert(key, value);
        }

        Ok(Response {
            raw_response: response.clone(),
            status_code: StatusCode::from_u16(status_code).unwrap(),
            headers: headers_map,
            body: body.to_string(),
            request_used: self.clone(),
        })
    }
}