pub mod builder;
pub use builder::ValidBuilder;

pub mod request;
pub use request::ValidRequest;

pub mod response;
pub use response::ValidResponse;

mod errors;

pub mod status_code;
pub use status_code::StatusCode;

mod tests {
    extern crate tokio;
    use super::*;
    use builder::Builder;

    use tokio::runtime::Runtime;

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
        let response = ok_builder.async_send();

        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(response);

        println!("{:?}", result);
    }
}
