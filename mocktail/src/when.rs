use std::{cell::Cell, rc::Rc};

use crate::{body::Body, headers::Headers, matchers, request::Method, Matcher};

/// A request match conditions builder.
#[derive(Default, Clone)]
pub struct When(Rc<Cell<Vec<Box<dyn Matcher>>>>);

impl When {
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(Vec::new())))
    }

    /// Sorts, deduplicates, and returns the inner set of matchers.
    pub fn into_inner(self) -> Vec<Box<dyn Matcher>> {
        let mut m = self.0.take();
        m.sort_unstable();
        m.dedup();
        m
    }

    /// Pushes a matcher to the set of matchers.
    fn push(&self, matcher: impl Matcher) {
        let mut m = self.0.take();
        m.push(Box::new(matcher));
        self.0.set(m);
    }

    /// Any.
    /// Should not be combined with other matchers.
    pub fn any(self) -> Self {
        self.push(matchers::any());
        self
    }

    /// HTTP method.
    pub fn method(self, method: impl Into<Method>) -> Self {
        self.push(matchers::method(method.into()));
        self
    }

    /// Path.
    pub fn path(self, path: impl Into<String>) -> Self {
        self.push(matchers::path(path.into()));
        self
    }

    /// Body.
    pub fn body(self, body: Body) -> Self {
        self.push(matchers::body(body));
        self
    }

    /// Headers.
    pub fn headers(self, headers: Headers) -> Self {
        self.push(matchers::headers(headers));
        self
    }

    /// Headers exact.
    pub fn headers_exact(self, headers: Headers) -> Self {
        self.push(matchers::headers_exact(headers));
        self
    }

    /// Header.
    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.push(matchers::header(name, value));
        self
    }

    /// Header exists.
    pub fn header_exists(self, name: impl Into<String>) -> Self {
        self.push(matchers::header_exists(name.into()));
        self
    }

    /// Custom matcher.
    pub fn matcher(self, matcher: impl Matcher) -> Self {
        self.push(matcher);
        self
    }
}

/// Body convenience methods.
impl When {
    /// Raw bytes body.
    pub fn raw(self, body: Vec<u8>) -> Self {
        self.push(matchers::body(Body::raw(body)));
        self
    }

    /// Json body.
    pub fn json(self, body: impl serde::Serialize) -> Self {
        self.push(matchers::body(Body::json(body)));
        self
    }

    /// Newline delimited JSON streaming body.
    pub fn json_lines_stream(
        self,
        messages: impl IntoIterator<Item = impl serde::Serialize>,
    ) -> Self {
        self.push(matchers::body(Body::json_lines_stream(messages)));
        self
    }

    /// Protobuf body.
    pub fn pb(self, body: impl prost::Message) -> Self {
        self.push(matchers::body(Body::pb(body)));
        self
    }

    /// Protobuf streaming body.
    pub fn pb_stream(self, messages: impl IntoIterator<Item = impl prost::Message>) -> Self {
        self.push(matchers::body(Body::pb_stream(messages)));
        self
    }
}

/// Method convenience methods.
impl When {
    /// HTTP GET method.
    pub fn get(self) -> Self {
        self.push(matchers::method(Method::GET));
        self
    }

    /// HTTP POST method.
    pub fn post(self) -> Self {
        self.push(matchers::method(Method::POST));
        self
    }

    /// HTTP PUT method.
    pub fn put(self) -> Self {
        self.push(matchers::method(Method::PUT));
        self
    }

    /// HTTP DELETE method.
    pub fn delete(self) -> Self {
        self.push(matchers::method(Method::DELETE));
        self
    }
}
