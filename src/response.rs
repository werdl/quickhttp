use std::collections::HashMap;
use crate::errors::RequestError;
use crate::request::Request;
use crate::status_code::StatusCode;
use crate::ValidRequest;
/// Describes the response of an HTTP request
pub trait ValidResponse {
    /// Resend the request that generated this response
    fn resend(&self) -> Result<Response, RequestError>;
}

/// Describes the response of an HTTP request, and contains the response data
#[derive(Clone)]
pub struct Response {
    /// The status code of the response
    pub status_code: StatusCode,

    /// The raw response, as a string
    pub raw_response: String,

    /// The headers of the response
    pub headers: HashMap<String, String>,

    /// The body of the response
    pub body: String,

    /// An exact copy of the request that generated this response
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
    /// Resend the request that generated this response
    fn resend(&self) -> Result<Response, RequestError> {
        let request = self.request_used.clone();
        request.send()
    }
}