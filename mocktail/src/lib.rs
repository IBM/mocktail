#![doc = include_str!("../README.md")]
mod body;
pub use body::Body;
mod headers;
pub use headers::*;
mod matchers;
pub use matchers::*;
mod mock;
pub use mock::Mock;
mod mock_set;
pub use mock_set::MockSet;
mod request;
pub use request::Request;
mod response;
pub use response::Response;
mod when;
pub use when::When;
mod then;
pub use then::Then;
mod server;
pub use server::MockServer;
mod buf_list;
mod ext;
mod service;
pub mod prelude {
    pub use crate::{
        matchers::*, Body, HeaderName, HeaderValue, Headers, Mock, MockServer, MockSet, Request,
        Response, Then, When,
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
