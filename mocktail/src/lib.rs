#![doc = include_str!("../README.md")]
pub mod body;
mod headers;
pub use headers::Headers;
pub mod matchers;
mod mock;
pub use mock::Mock;
pub mod mock_builder;
mod mock_set;
pub use mock_set::MockSet;
mod request;
pub use request::{Method, Request};
mod response;
pub use response::{Response, StatusCode};
pub mod server;
pub mod prelude {
    pub use crate::{
        body::Body,
        headers::Headers,
        matchers::*,
        mock::Mock,
        mock_set::MockSet,
        request::{Method, Request},
        response::{Response, StatusCode},
        server::MockServer,
    };
}
mod ext;
mod service;

/// Represents errors that can occur while serving mocks.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid: {0}")]
    Invalid(String),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("server error: {0}")]
    ServerError(String),
}
