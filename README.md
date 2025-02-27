# mocktail

mocktail is a minimal framework for mocking HTTP and gRPC services in Rust with support for streaming.

<!-- [![Crates.io](https://img.shields.io/crates/v/mocktail)](https://crates.io/crates/mocktail)
[![Documentation](https://docs.rs/mocktail/badge.svg)](https://docs.rs/mocktail)
[![Crates.io](https://img.shields.io/crates/l/mocktail)](LICENSE) -->

# Table of contents
* [Features](#features)
* [Usage](#usage)
* [Examples](#examples)

# Features
- Mocks HTTP and gRPC servers
- Mocks defined in Rust using a simple API
- Supports HTTP streaming
- Supports gRPC unary, client-streaming, server-streaming, and bidirectional-streaming methods
- Performs basic "full body" (equals) matching

# Concepts
## Mock Server
A server that handles mock requests. This crate contains 2 servers:
- HttpMockServer
- GrpcMockServer 

## Mock Body
An enum containing the bytes of a mock request or response body.

```rust
pub enum MockBody {
    Empty,
    Full(Bytes),
    Stream(Vec<Bytes>),
}
```

## Mock Request
A mock request containing an optional body and optional headers.

```rust
// An empty body
MockRequest::empty()
// With a body that implements `Into<Bytes>`
MockRequest::new(body)
MockRequest::stream(messages)
// Convenience constructors:
// JSON: with a body that implements `serde::Serialize`
MockRequest::json(body)
MockRequest::json_stream(messages)
// Protobuf (gRPC): with a body that implements `prost::Message`
MockRequest::pb(body)
MockRequest::pb_stream(messages)
```

## Mock Response

A mock response containing a response code, optional body, optional headers, and optional error message. The response code defaults to `200`.

```rust
// An empty body
MockResponse::empty()
// With a body that implements `Into<Bytes>`
MockResponse::new(body)
MockResponse::stream(messages)
// Convenience constructors:
// JSON: with a body that implements `serde::Serialize`
MockResponse::json(body)
MockResponse::json_stream(messages)
// Protobuf (gRPC): with a body that implements `prost::Message`
MockResponse::pb(body)
MockResponse::pb_stream(messages)
```

## Mock
A mock request and response pair.

## Mock Path
A mock path for request matching.

## Mock Set
A set of mocks for a service.

# Usage
1. Add `mocktail` to `Cargo.toml` as a development dependency:
    ```toml
    [dev-dependencies]
    mocktail = { git = "https://github.com/IBM/mocktail.git", version = "0.1.1-alpha" }
    ```

2. In a test context, use as follows. See [mocktail-test](/mocktail-test/) crate for more usage examples.

    ```rust
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HelloRequest {
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HelloResponse {
        pub message: String,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use mocktail::prelude::*;

        #[tokio::test]
        async fn test_hello_simple() -> Result<(), Box<dyn std::error::Error>> {
            // Create a mock set.
            let mut mocks = MockSet::new();
            // Insert mocks.
            // `MockRequest::json()` and `MockResponse::json()` are convenience methods 
            // that handle JSON serialization to avoid `serde_json::to_vec(&value)` boilerplate.
            mocks.insert(
                MockPath::new(Method::POST, "/hello"),
                Mock::new(
                    MockRequest::json(HelloRequest { name: "World".into() }),
                    MockResponse::json(HelloResponse {
                        message: "Hello World!".into(),
                    }),
                ),
            );
            // Create and start a mock server.
            let server = HttpMockServer::new("hello", mocks)?;
            server.start().await?;

            // Send request to mock server.
            let client = reqwest::Client::new();
            let response = client
                .post(server.url("/hello"))
                .json(&HelloRequest { name: "World".into() })
                .send()
                .await?;
            assert!(response.status() == StatusCode::OK);
            let body = response.json::<HelloResponse>().await?;
            dbg!(&body);

            // Send request to mock server. 
            // Doesn't match a mock so should return an error.
            let client = reqwest::Client::new();
            let response = client
                .post(server.url("/hello"))
                .json(&HelloRequest {
                    name: "Missing".into(),
                })
                .send()
                .await?;
            assert!(response.status() == StatusCode::NOT_FOUND);

            Ok(())
        }
    }
    ```

# Examples
See [mocktail-test](/mocktail-test/) crate.
