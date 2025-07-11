![default-monochrome](https://github.com/user-attachments/assets/dcf68c3e-4c16-4a96-a6d3-2af4710692c6)

mocktail is a **minimal** crate for mocking HTTP and gRPC servers in Rust, with native support for streaming.

[![Crates.io](https://img.shields.io/crates/v/mocktail)](https://crates.io/crates/mocktail)
[![Documentation](https://docs.rs/mocktail/badge.svg)](https://docs.rs/mocktail)
[![Book](https://img.shields.io/static/v1?label=mocktail&message=user-guide&color=153292)](https://ibm.github.io/mocktail/)
[![Crates.io](https://img.shields.io/crates/l/mocktail)](LICENSE)


# Table of contents
* [Features](#features)
* [Getting Started](#getting-started)
* [Examples](#examples)

# Features
- Mocks HTTP and gRPC servers
- Mocks defined in Rust using a simple, ergonomic API
- Provides first-class support for streaming
- Supports gRPC unary, client-streaming, server-streaming, and bidirectional-streaming methods
- Match requests to mock responses using built-in matchers or custom matchers
- Fully asynchronous

# Getting Started
1. Add `mocktail` to `Cargo.toml` as a development dependency:
    ```toml
    [dev-dependencies]
    mocktail = "0.2.6-alpha"
    ```

2. Basic usage example:
    ```rust
    use mocktail::prelude::*;

    #[tokio::test]
    async fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        // Create a mock set
        let mut mocks = MockSet::new();

        // Build a mock that returns a "hello world!" response
        // to POST requests to the /hello endpoint with the text "world"
        // in the body.
        mocks.mock(|when, then| {
            when.post().path("/hello").text("world");
            then.text("hello world!");
        });

        // Create and start a mock server
        let mut server = MockServer::new_http("example").with_mocks(mocks);
        server.start().await?;

        // Create a client
        let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

        // Send a request that matches the mock created above
        let response = client
            .post(server.url("/hello"))
            .body("world")
            .send()
            .await?;
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = response.text().await?;
        assert_eq!(body, "hello world!");

        // Send a request that doesn't match a mock
        let response = client.get(server.url("/nope")).send().await?;
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

        // Mocks can also be registered to the server directly
        // Register a mock that will match the request above that returned 404
        server.mock(|when, then| {
            when.get().path("/nope");
            then.text("yep!");
        });

        // Send the request again, it should now match
        let response = client.get(server.url("/nope")).send().await?;
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = response.text().await?;
        assert_eq!(body, "yep!");

        // Mocks can be cleared from the server, enabling server reuse
        server.mocks.clear();

        Ok(())
    }
    ```

3. See the [book](https://ibm.github.io/mocktail/) and [examples](/mocktail-tests/tests/examples) in the `mocktail-tests` crate.

# Examples
See [examples](/mocktail-tests/tests/examples) in the `mocktail-tests` crate.

# Related projects
This crate takes inspiration from other great mocking libraries including:
- [wiremock](https://github.com/wiremock/wiremock)
- [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs)
- [httpmock](https://github.com/alexliesenfeld/httpmock)
