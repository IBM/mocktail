mod body;
use std::num::NonZeroU16;

pub use body::{HyperBoxBody, MockBody, TonicBoxBody};

mod set;
use bytes::Bytes;
pub use set::{MockPath, MockSet};

mod request;
pub use request::MockRequest;

mod response;
pub use response::MockResponse;

use crate::{utils::tonic::CodeExt, Error};

/// A mock request and response pair.
#[derive(Debug, Clone)]
pub struct Mock {
    pub request: MockRequest,
    pub response: MockResponse,
}

impl Mock {
    /// Creates a new [`Mock`].
    pub fn new(request: MockRequest, response: MockResponse) -> Self {
        Self { request, response }
    }
}

/// An HTTP method.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl PartialEq<Method> for http::Method {
    fn eq(&self, other: &Method) -> bool {
        self.to_string().to_uppercase() == other.to_string().to_uppercase()
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(format!("Invalid HTTP method {}", input)),
        }
    }
}

impl From<http::Method> for Method {
    fn from(value: http::Method) -> Self {
        match value.as_str() {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "OPTIONS" => Self::OPTIONS,
            "HEAD" => Self::HEAD,
            "TRACE" => Self::TRACE,
            "CONNECT" => Self::CONNECT,
            "PATCH" => Self::PATCH,
            _ => unimplemented!(),
        }
    }
}

/// HTTP headers.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Headers(Vec<(String, String)>);

impl Headers {
    pub fn new(headers: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>) -> Self {
        Self(
            headers
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }

    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.push((key.into(), value.into()));
    }

    pub fn get(&self, key: &str) -> Option<&(String, String)> {
        self.0.iter().find(|(k, _)| k == key)
    }

    pub fn remove(&mut self, key: &str) -> Option<(String, String)> {
        if let Some(index) = self.0.iter().position(|(k, _)| k == key) {
            Some(self.0.swap_remove(index))
        } else {
            None
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }

    pub fn contains(&self, value: (&str, &str)) -> bool {
        self.0.iter().any(|(k, v)| k == value.0 && v == value.1)
    }

    /// Returns `true` if the headers are a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    pub fn is_subset(&self, other: &Headers) -> bool {
        self.0.iter().all(|value| other.0.contains(value))
    }

    /// Returns `true` if the headers are a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    pub fn is_superset(&self, other: &Headers) -> bool {
        other.is_subset(self)
    }
}

impl From<http::HeaderMap> for Headers {
    fn from(value: http::HeaderMap) -> Self {
        Self(
            value
                .into_iter()
                .map(|(key, value)| {
                    let key = key.unwrap().to_string();
                    let value = value.to_str().unwrap().to_string();
                    (key, value)
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Headers> for http::HeaderMap {
    fn from(value: Headers) -> Self {
        value
            .0
            .into_iter()
            .map(|(key, value)| {
                let header_name = http::HeaderName::try_from(key).unwrap();
                let header_value = http::HeaderValue::try_from(value).unwrap();
                (header_name, header_value)
            })
            .collect()
    }
}

impl From<Headers> for tonic::metadata::MetadataMap {
    fn from(value: Headers) -> Self {
        tonic::metadata::MetadataMap::from_headers(value.into())
    }
}

/// An HTTP status code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

impl StatusCode {
    pub fn from_u16(src: u16) -> Result<StatusCode, Error> {
        if !(100..1000).contains(&src) {
            return Err(Error::Invalid("invalid status code".into()));
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(|| Error::Invalid("invalid status code".into()))
    }

    pub fn as_u16(&self) -> u16 {
        self.0.get()
    }

    pub fn is_informational(&self) -> bool {
        200 > self.0.get() && self.0.get() >= 100
    }

    /// Check if status is within 200-299.
    pub fn is_success(&self) -> bool {
        300 > self.0.get() && self.0.get() >= 200
    }

    /// Check if status is success
    pub fn is_ok(&self) -> bool {
        self.is_success()
    }

    /// Check if status is within 300-399.
    pub fn is_redirection(&self) -> bool {
        400 > self.0.get() && self.0.get() >= 300
    }

    /// Check if status is within 400-499.
    pub fn is_client_error(&self) -> bool {
        500 > self.0.get() && self.0.get() >= 400
    }

    /// Check if status is within 500-599.
    pub fn is_server_error(&self) -> bool {
        600 > self.0.get() && self.0.get() >= 500
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        Self(NonZeroU16::new(200).unwrap())
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

/// A mock SSE event.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub event: Option<String>,
    pub data: String,
}

impl Event {
    pub fn new(data: impl Into<String>) -> Self {
        Self {
            event: None,
            data: data.into(),
        }
    }

    pub fn with_event(mut self, event: impl Into<String>) -> Self {
        self.event = Some(event.into());
        self
    }
}

impl From<Event> for Bytes {
    fn from(value: Event) -> Self {
        let mut s: String = String::new();
        if let Some(event) = value.event {
            s.push_str(&format!("event: {event}\n"));
        }
        s.push_str(&format!("data: {}\n\n", value.data));
        s.into()
    }
}
