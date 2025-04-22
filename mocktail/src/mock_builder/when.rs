//! When
use std::{cell::Cell, rc::Rc, sync::Arc};

use bytes::Bytes;

use crate::{
    body::Body,
    headers::{HeaderName, HeaderValue, Headers},
    matchers,
    matchers::Matcher,
    request::Method,
};

/// A request match conditions builder.
#[derive(Default, Clone)]
pub struct When(Rc<Cell<Vec<Arc<dyn Matcher>>>>);

impl When {
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(Vec::new())))
    }

    /// Sorts, deduplicates, and returns the inner set of matchers.
    pub fn into_inner(self) -> Vec<Arc<dyn Matcher>> {
        let mut m = self.0.take();
        m.sort_unstable();
        m.dedup();
        m
    }

    /// Pushes a matcher to the set of matchers.
    fn push(&self, matcher: impl Matcher) {
        let mut m = self.0.take();
        m.push(Arc::new(matcher));
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
        self.push(matchers::path(path));
        self
    }

    /// Path prefix.
    pub fn path_prefix(self, prefix: impl Into<String>) -> Self {
        self.push(matchers::path_prefix(prefix));
        self
    }

    /// Body.
    pub fn body(self, body: Body) -> Self {
        self.push(matchers::body(body));
        self
    }

    /// Headers.
    ///
    /// Cannonicalizes passed in header name and values to ensure matching is done in a
    /// case-insensitive manner as required by [RFC2616][rfc-headers]
    ///
    /// [rfc-headers]: https://www.rfc-editor.org/rfc/rfc2616#section-4.2
    pub fn headers<T, U>(self, headers: impl IntoIterator<Item = (T, U)>) -> Self
    where
        T: Into<HeaderName>,
        U: Into<HeaderValue>,
    {
        self.push(matchers::headers(Headers::from_iter(headers)));
        self
    }

    /// Headers exact.
    pub fn headers_exact<T, U>(self, headers: impl IntoIterator<Item = (T, U)>) -> Self
    where
        T: Into<HeaderName>,
        U: Into<HeaderValue>,
    {
        self.push(matchers::headers_exact(Headers::from_iter(headers)));
        self
    }

    /// Header.
    pub fn header(self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.push(matchers::header(name, value));
        self
    }

    /// Header exists.
    pub fn header_exists(self, name: impl Into<String>) -> Self {
        self.push(matchers::header_exists(name));
        self
    }

    /// Query params.
    pub fn query_params(
        self,
        pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.push(matchers::query_params(pairs));
        self
    }

    /// Query param.
    pub fn query_param(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.push(matchers::query_param(key, value));
        self
    }

    /// Query param exists.
    pub fn query_param_exists(self, key: impl Into<String>) -> Self {
        self.push(matchers::query_param_exists(key));
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
    /// Empty body.
    pub fn empty(self) -> Self {
        self.push(matchers::body(Body::empty()));
        self
    }

    /// Raw bytes body.
    pub fn bytes(self, body: impl Into<Bytes>) -> Self {
        self.push(matchers::body(Body::bytes(body.into())));
        self
    }

    /// Raw bytes stream body.
    pub fn bytes_stream(self, messages: impl IntoIterator<Item = impl Into<Bytes>>) -> Self {
        self.push(matchers::body(Body::bytes_stream(messages)));
        self
    }

    /// Text body.
    pub fn text(self, body: impl Into<String>) -> Self {
        let body: String = body.into();
        self.push(matchers::body(Body::bytes(body)));
        self
    }

    /// Text stream body.
    pub fn text_stream(self, messages: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let messages = messages.into_iter().map(|msg| {
            let msg: String = msg.into();
            msg
        });
        self.push(matchers::body(Body::bytes_stream(messages)));
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

    /// HTTP HEAD method.
    pub fn head(self) -> Self {
        self.push(matchers::method(Method::HEAD));
        self
    }
}
