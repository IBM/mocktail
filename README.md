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
- Mocks defined in Rust using a simple, ergonomic API
- Supports HTTP streaming
- Supports gRPC unary, client-streaming, server-streaming, and bidirectional-streaming methods
- Performs matching using built-in matchers or custom matchers

# Usage
1. Add `mocktail` to `Cargo.toml` as a development dependency:
    ```toml
    [dev-dependencies]
    mocktail = { git = "https://github.com/IBM/mocktail.git", version = "0.2.0-alpha" }
    ```

2. See [mocktail-tests](/mocktail-tests/) crate for usage examples.

# Examples
See [mocktail-tests](/mocktail-tests/) crate.
