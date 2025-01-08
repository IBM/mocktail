#![doc = include_str!("../README.md")]
pub mod codegen;
pub mod mock;
pub mod server;
pub mod utils;
pub mod prelude {
    pub use crate::generate_grpc_server;
    pub use crate::mock::{Mock, MockBody, MockPath, MockRequest, MockResponse, MockSet};
    pub use crate::server::HttpMockServer;
    pub use crate::utils::prost::MessageExt as _;
    pub use crate::Error;
    pub use http::{HeaderMap, HeaderValue, Method, StatusCode};
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
