use std::collections::HashMap;
use crate::errors::RequestError;
use crate::request::Request;
use crate::status_code::StatusCode;
use crate::ValidRequest;
pub trait ValidResponse {
    fn resend(&self) -> Result<Response, RequestError>;
}

#[derive(Clone)]
pub struct Response {
    pub status_code: StatusCode,
    pub raw_response: String,

    pub headers: HashMap<String, String>,
    pub body: String,

    pub request_used: Request,
}

impl core::fmt::Debug for Response {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.raw_response)
    }
}

impl core::fmt::Display for Response {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.raw_response)
    }
}

impl ValidResponse for Response {
    fn resend(&self) -> Result<Response, RequestError> {
        let request = self.request_used.clone();
        request.send()
    }
}