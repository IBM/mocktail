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

## Mock Request
A mock request containing an optional body and optional headers.

```rust
// With an empty body
MockRequest::empty()

// With a body that implements `serde::Serialize` (for HTTP) or `prost::Message` (for gRPC)
MockRequest::new(body)
// alt: MockRequest::full(body)

// With an iterator of messages that implement `serde::Serialize` (for HTTP) or `prost::Message` (for gRPC)
MockRequest::stream(messages)
```

## Mock Response
A mock response containing a response code, optional body, optional headers, and optional error message. The response code defaults to `200`.

```rust
// With an empty body
MockResponse::empty()

// With a body that implements `serde::Serialize` (for HTTP) or `prost::Message` (for gRPC)
MockResponse::new(body)
// alt: MockRequest::full(body)

// With an iterator of messages that implement `serde::Serialize` (for HTTP) or `prost::Message` (for gRPC)
MockResponse::stream(messages)

// With an iterator of [`Event`] messages.
MockResponse::sse_stream(messages)
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

2. See [mocktail-test](/mocktail-test/) crate for usage examples.


# Examples
See [mocktail-test](/mocktail-test/) crate.
