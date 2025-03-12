# mocktail
[![Crates.io](https://img.shields.io/crates/v/mocktail)](https://crates.io/crates/mocktail)
[![Documentation](https://docs.rs/mocktail/badge.svg)](https://docs.rs/mocktail)
[![Crates.io](https://img.shields.io/crates/l/mocktail)](LICENSE)

mocktail is a **minimal** crate for mocking HTTP and gRPC servers in Rust with support for streaming.

# Table of contents
* [Features](#features)
* [Getting Started](#getting-started)
* [Examples](#examples)

# Features
- Mocks HTTP and gRPC servers
- Mocks defined in Rust using a simple, ergonomic API
- Supports HTTP streaming
- Supports gRPC unary, client-streaming, server-streaming, and bidirectional-streaming methods
- Match requests to mock responses using built-in matchers or custom matchers

# Getting Started
1. Add `mocktail` to `Cargo.toml` as a development dependency:
    ```toml
    [dev-dependencies]
    mocktail = { git = "https://github.com/IBM/mocktail.git", version = "0.2.1-alpha" }
    ```

2. For now, see [examples](/mocktail-tests/tests/examples) in the `mocktail-tests` crate. Additional documentation coming soon.

# Examples
See [examples](/mocktail-tests/tests/examples) in the `mocktail-tests` crate.

# Related projects
This crate takes inspiration from other great mocking libraries including:
- [wiremock](https://github.com/wiremock/wiremock)
- [wiremock-rs](https://github.com/LukeMathWalker/wiremock-rs)
- [httpmock](https://github.com/alexliesenfeld/httpmock)
