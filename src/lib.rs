//! # http-client
//! > A small library for making HTTP requests
//! Designed to be a small, simple, and easy to use library for making HTTP requests.
//! Supports all key features of HTTP
//! Can be used synchronously or asynchronously (for example with `tokio`)


/// Request Builder type, allowing creation of a request
pub mod builder;
/// import the builder trait, to make the exposed builder trait available
pub use builder::ValidBuilder;

/// Request type, designed to be generated from a builder
pub mod request;
/// import the request trait, to make the exposed request trait available
pub use request::ValidRequest;

/// Response type, designed to be generated from a request
pub mod response;
/// import the response trait, to make the exposed response trait available
pub use response::ValidResponse;

/// Error type, designed to be generated from a response
mod errors;

/// status code types, designed to be used in the response field
pub mod status_code;
/// import the status code enum
pub use status_code::StatusCode;

#[cfg(test)]
mod tests {
    use super::*;
    use builder::Builder;

    #[test]
    fn test_builder() {
        let builder = Builder::new()
            .uri("http://httpbin.org/post?name=bob".to_string())
            .method("POST".to_string())
            .body("name=John".to_string())
            .header(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            )
            .build();

        let ok_builder = builder.unwrap();
        let response = ok_builder.send();


        println!("{:?}", response);
    }
}
