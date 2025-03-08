use bytes::Bytes;

use super::headers::Headers;

/// A representation of a HTTP request.
#[derive(Debug, Clone, PartialEq)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
    pub headers: Headers,
    pub body: Bytes,
}

impl Request {
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            method,
            path: path.into(),
            query: None,
            headers: Headers::default(),
            body: Bytes::default(),
        }
    }

    pub fn from_parts(parts: http::request::Parts) -> Self {
        Self {
            method: parts.method.into(),
            path: parts.uri.path().to_string(),
            query: parts.uri.query().map(Into::into),
            headers: parts.headers.into(),
            body: Bytes::default(),
        }
    }

    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn with_body(mut self, body: Bytes) -> Self {
        self.body = body;
        self
    }
}

/// A representation of an HTTP method.
#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Method {
    #[default]
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

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(format!("Invalid HTTP method {}", value)),
        }
    }
}

impl TryFrom<&str> for Method {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(format!("Invalid HTTP method {}", value)),
        }
    }
}

impl From<http::Method> for Method {
    fn from(value: http::Method) -> Self {
        match value.as_str() {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => unimplemented!(),
        }
    }
}
