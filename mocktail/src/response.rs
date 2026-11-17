//! Mock response
use std::time::Duration;

use super::{body::Body, headers::Headers, status::StatusCode};

/// Represents a HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Body,
    pub message: Option<String>,
    pub delay: Option<Duration>,
    pub chunk_delay: Option<Duration>,
}

impl Response {
    pub fn new(body: impl Into<Body>) -> Self {
        Self {
            status: StatusCode::default(),
            headers: Headers::default(),
            body: body.into(),
            message: None,
            delay: None,
            chunk_delay: None,
        }
    }

    pub fn with_status(mut self, status: impl Into<StatusCode>) -> Self {
        self.status = status.into();
        self
    }

    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    pub fn with_chunk_delay(mut self, delay: Duration) -> Self {
        self.chunk_delay = Some(delay);
        self
    }

    pub fn status(&self) -> &StatusCode {
        &self.status
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn delay(&self) -> Option<Duration> {
        self.delay
    }

    pub fn chunk_delay(&self) -> Option<Duration> {
        self.chunk_delay
    }

    pub fn is_ok(&self) -> bool {
        self.status.is_ok()
    }

    pub fn is_error(&self) -> bool {
        self.status.is_error()
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            headers: Headers::default(),
            body: Body::default(),
            message: None,
            delay: None,
            chunk_delay: None,
        }
    }
}
