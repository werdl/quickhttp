/// Trait defining the error type for the library
pub trait Error {
    fn new(message: String) -> Self;
}

/// Error type for a builder (for example, if a required field is missing)
#[derive(Clone, Debug)]
pub struct BuilderError {
    pub(crate) message: String,
}

impl Error for BuilderError {
    fn new(message: String) -> BuilderError {
        BuilderError { message }
    }
}

/// Error type for a request (for example, if the request fails)
#[derive(Clone, Debug)]
pub struct RequestError {
    pub(crate) message: String,
}

impl Error for RequestError {
    fn new(message: String) -> RequestError {
        RequestError { message }
    }
}

/// Error type for a response (for example, if the response is invalid)
#[derive(Clone, Debug)]
pub struct ResponseError {
    pub(crate) message: String,
}

impl Error for ResponseError {
    fn new(message: String) -> ResponseError {
        ResponseError { message }
    }
}

impl core::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "BuilderError: {}", self.message)
    }
}

impl core::fmt::Display for RequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "RequestError: {}", self.message)
    }
}

impl core::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "ResponseError: {}", self.message)
    }
}