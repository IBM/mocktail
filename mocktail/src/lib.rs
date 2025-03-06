#![doc = include_str!("../README.md")]
pub mod mock;
pub mod server;
pub mod utils;
pub mod prelude {
    pub use http::{HeaderMap, HeaderValue, Method, StatusCode};

    pub use crate::{
        mock::{Mock, MockBody, MockPath, MockRequest, MockResponse, MockSet},
        server::{GrpcMockServer, HttpMockServer, MockServer},
        utils::prost::MessageExt as _,
        Error,
    };
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid: {0}")]
    Invalid(String),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
