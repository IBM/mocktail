//! Mock response
use std::num::NonZeroU16;

use super::{body::Body, headers::Headers};
use crate::{ext::CodeExt, Error};

/// Represents a HTTP response.
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

/// Represents a HTTP status code.
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
    pub const CONTINUE: StatusCode = StatusCode(NonZeroU16::new(100).unwrap());
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(NonZeroU16::new(101).unwrap());
    pub const PROCESSING: StatusCode = StatusCode(NonZeroU16::new(102).unwrap());
    pub const EARLY_HINTS: StatusCode = StatusCode(NonZeroU16::new(103).unwrap());

    pub const OK: StatusCode = StatusCode(NonZeroU16::new(200).unwrap());
    pub const CREATED: StatusCode = StatusCode(NonZeroU16::new(201).unwrap());
    pub const ACCEPTED: StatusCode = StatusCode(NonZeroU16::new(202).unwrap());
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode = StatusCode(NonZeroU16::new(203).unwrap());
    pub const NO_CONTENT: StatusCode = StatusCode(NonZeroU16::new(204).unwrap());
    pub const RESET_CONTENT: StatusCode = StatusCode(NonZeroU16::new(205).unwrap());
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(NonZeroU16::new(206).unwrap());
    pub const MULTI_STATUS: StatusCode = StatusCode(NonZeroU16::new(207).unwrap());
    pub const ALREADY_REPORTED: StatusCode = StatusCode(NonZeroU16::new(208).unwrap());
    pub const IM_USED: StatusCode = StatusCode(NonZeroU16::new(226).unwrap());

    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(NonZeroU16::new(300).unwrap());
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(NonZeroU16::new(301).unwrap());
    pub const FOUND: StatusCode = StatusCode(NonZeroU16::new(302).unwrap());
    pub const SEE_OTHER: StatusCode = StatusCode(NonZeroU16::new(303).unwrap());
    pub const NOT_MODIFIED: StatusCode = StatusCode(NonZeroU16::new(304).unwrap());
    pub const USE_PROXY: StatusCode = StatusCode(NonZeroU16::new(305).unwrap());
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode(NonZeroU16::new(307).unwrap());
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode(NonZeroU16::new(308).unwrap());

    pub const BAD_REQUEST: StatusCode = StatusCode(NonZeroU16::new(400).unwrap());
    pub const UNAUTHORIZED: StatusCode = StatusCode(NonZeroU16::new(401).unwrap());
    pub const PAYMENT_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(402).unwrap());
    pub const FORBIDDEN: StatusCode = StatusCode(NonZeroU16::new(403).unwrap());
    pub const NOT_FOUND: StatusCode = StatusCode(NonZeroU16::new(404).unwrap());
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(NonZeroU16::new(405).unwrap());
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(NonZeroU16::new(406).unwrap());
    pub const PROXY_AUTHENTICATION_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(407).unwrap());
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(NonZeroU16::new(408).unwrap());
    pub const CONFLICT: StatusCode = StatusCode(NonZeroU16::new(409).unwrap());
    pub const GONE: StatusCode = StatusCode(NonZeroU16::new(410).unwrap());
    pub const LENGTH_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(411).unwrap());
    pub const PRECONDITION_FAILED: StatusCode = StatusCode(NonZeroU16::new(412).unwrap());
    pub const PAYLOAD_TOO_LARGE: StatusCode = StatusCode(NonZeroU16::new(413).unwrap());
    pub const URI_TOO_LONG: StatusCode = StatusCode(NonZeroU16::new(414).unwrap());
    pub const UNSUPPORTED_MEDIA_TYPE: StatusCode = StatusCode(NonZeroU16::new(415).unwrap());
    pub const RANGE_NOT_SATISFIABLE: StatusCode = StatusCode(NonZeroU16::new(416).unwrap());
    pub const EXPECTATION_FAILED: StatusCode = StatusCode(NonZeroU16::new(417).unwrap());
    pub const IM_A_TEAPOT: StatusCode = StatusCode(NonZeroU16::new(418).unwrap());
    pub const MISDIRECTED_REQUEST: StatusCode = StatusCode(NonZeroU16::new(421).unwrap());
    pub const UNPROCESSABLE_ENTITY: StatusCode = StatusCode(NonZeroU16::new(422).unwrap());
    pub const LOCKED: StatusCode = StatusCode(NonZeroU16::new(423).unwrap());
    pub const FAILED_DEPENDENCY: StatusCode = StatusCode(NonZeroU16::new(424).unwrap());
    pub const TOO_EARLY: StatusCode = StatusCode(NonZeroU16::new(425).unwrap());
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(426).unwrap());
    pub const PRECONDITION_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(428).unwrap());
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(NonZeroU16::new(429).unwrap());
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: StatusCode = StatusCode(NonZeroU16::new(431).unwrap());
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: StatusCode = StatusCode(NonZeroU16::new(451).unwrap());

    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(NonZeroU16::new(500).unwrap());
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(NonZeroU16::new(501).unwrap());
    pub const BAD_GATEWAY: StatusCode = StatusCode(NonZeroU16::new(502).unwrap());
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(NonZeroU16::new(503).unwrap());
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(NonZeroU16::new(504).unwrap());
    pub const HTTP_VERSION_NOT_SUPPORTED: StatusCode = StatusCode(NonZeroU16::new(505).unwrap());
    pub const VARIANT_ALSO_NEGOTIATES: StatusCode = StatusCode(NonZeroU16::new(506).unwrap());
    pub const INSUFFICIENT_STORAGE: StatusCode = StatusCode(NonZeroU16::new(507).unwrap());
    pub const LOOP_DETECTED: StatusCode = StatusCode(NonZeroU16::new(508).unwrap());
    pub const NOT_EXTENDED: StatusCode = StatusCode(NonZeroU16::new(510).unwrap());
    pub const NETWORK_AUTHENTICATION_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(511).unwrap());
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
