# quickhttp
> A simple HTTP client for Rust, with no dependencies.

[![GitHub Actions][gh-image]][gh-checks]
[![crates.io][cratesio-image]][cratesio]
[![thetime on docs.rs][docsrs-image]][docsrs]

[gh-image]: https://github.com/werdl/quickhttp/actions/workflows/rust.yml/badge.svg
[gh-checks]: https://github.com/werdl/quickhttp/actions?query=workflow%20rust
[cratesio-image]: https://img.shields.io/crates/v/quickhttp.svg
[cratesio]: https://crates.io/crates/quickhttp
[docsrs-image]: https://docs.rs/quickhttp/badge.svg
[docsrs]: https://docs.rs/quickhttp

# Example
```rust
let res = Builder::new()
    .uri("http://httpbin.org/ip".to_string())
    .method("GET".to_string())
    .build()
    .unwrap()
    .send()
    .unwrap();


println!("{:?}", res.body);
```