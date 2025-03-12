use std::num::NonZeroU16;

use super::{body::Body, headers::Headers};
use crate::{ext::CodeExt, Error};

/// A representation of a HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Body,
    pub message: Option<String>,
}

impl Response {
    pub fn new(body: impl Into<Body>) -> Self {
        Self {
            status: StatusCode::default(),
            headers: Headers::default(),
            body: body.into(),
            message: None,
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
        }
    }
}

/// A representation of a HTTP status code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

impl StatusCode {
    pub fn from_u16(code: u16) -> Result<Self, Error> {
        if !(100..1000).contains(&code) {
            return Err(Error::Invalid("invalid status code".into()));
        }
        Ok(Self(NonZeroU16::new(code).unwrap()))
    }

    pub fn as_u16(&self) -> u16 {
        self.0.get()
    }

    pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.as_u16())
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.as_u16())
    }

    pub fn is_redirection(&self) -> bool {
        (300..400).contains(&self.as_u16())
    }

    pub fn is_error(&self) -> bool {
        (400..600).contains(&self.as_u16())
    }

    pub fn is_ok(&self) -> bool {
        self.is_success()
    }

    pub fn as_http(&self) -> http::StatusCode {
        http::StatusCode::from_u16(self.as_u16()).unwrap()
    }

    pub fn as_grpc(&self) -> tonic::Code {
        tonic::Code::from_u16(self.as_u16()).unwrap()
    }

    pub fn as_grpc_i32(&self) -> i32 {
        self.as_grpc() as i32
    }
}

impl StatusCode {
    pub const OK: StatusCode = StatusCode(NonZeroU16::new(200).unwrap());
    pub const NOT_FOUND: StatusCode = StatusCode(NonZeroU16::new(404).unwrap());
    pub const BAD_REQUEST: StatusCode = StatusCode(NonZeroU16::new(400).unwrap());
    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(NonZeroU16::new(500).unwrap());
    // TODO: add additional
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::OK
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl From<StatusCode> for u16 {
    fn from(status: StatusCode) -> u16 {
        status.0.get()
    }
}

impl From<http::StatusCode> for StatusCode {
    fn from(value: http::StatusCode) -> Self {
        Self::from_u16(value.as_u16()).unwrap()
    }
}

impl From<StatusCode> for http::StatusCode {
    fn from(value: StatusCode) -> Self {
        Self::from_u16(value.0.into()).unwrap()
    }
}

impl From<StatusCode> for tonic::Code {
    fn from(value: StatusCode) -> Self {
        tonic::Code::from_u16(value.as_u16()).unwrap()
    }
}
