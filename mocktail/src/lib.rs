#![doc = include_str!("../README.md")]
pub(crate) mod body;
pub(crate) mod headers;
pub(crate) mod matchers;
pub(crate) use matchers::*;
pub(crate) mod mock;
pub(crate) use mock::*;
pub(crate) mod mock_set;
pub(crate) use mock_set::*;
pub(crate) mod request;
pub(crate) use request::Request;
pub(crate) mod response;
pub(crate) use response::Response;
pub(crate) mod mock_builder;
pub(crate) mod server;
pub(crate) use mock_builder::*;
pub(crate) mod utils;
pub mod prelude {
    pub use crate::{
        body::Body, headers::Headers, matchers::*, mock::Mock, mock_set::MockSet, request::*,
        response::*, server::*,
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
    #[error("server error: {0}")]
    ServerError(String),
}
