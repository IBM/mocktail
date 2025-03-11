use super::{body::Body, headers::Headers, request::Request};
use crate::request::Method;
use std::cmp::Ordering;

/// A matcher.
pub trait Matcher: std::fmt::Debug + Send + Sync + 'static {
    /// Matcher name.
    fn name(&self) -> &str;
    /// Evaluates a match condition.
    fn matches(&self, req: &Request) -> bool;
}

impl PartialEq for dyn Matcher {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Eq for dyn Matcher {}

impl PartialOrd for dyn Matcher {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dyn Matcher {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name().cmp(other.name())
    }
}

/// Any matcher.
#[derive(Debug)]
pub struct AnyMatcher;

impl Matcher for AnyMatcher {
    fn name(&self) -> &str {
        "any"
    }
    fn matches(&self, _req: &Request) -> bool {
        true
    }
}

pub fn any() -> AnyMatcher {
    AnyMatcher
}

/// HTTP method matcher.
#[derive(Debug)]
pub struct MethodMatcher(Method);

impl Matcher for MethodMatcher {
    fn name(&self) -> &str {
        "method"
    }
    fn matches(&self, req: &Request) -> bool {
        req.method == self.0
    }
}

pub fn method(method: Method) -> MethodMatcher {
    MethodMatcher(method)
}

/// Path matcher.
#[derive(Debug)]
pub struct PathMatcher(String);

impl Matcher for PathMatcher {
    fn name(&self) -> &str {
        "path"
    }
    fn matches(&self, req: &Request) -> bool {
        req.path == self.0
    }
}

pub fn path(path: String) -> PathMatcher {
    PathMatcher(path)
}

/// Body matcher.
#[derive(Debug)]
pub struct BodyMatcher(Body);

impl Matcher for BodyMatcher {
    fn name(&self) -> &str {
        "body"
    }
    fn matches(&self, req: &Request) -> bool {
        self.0 == req.body
    }
}

pub fn body(body: Body) -> BodyMatcher {
    BodyMatcher(body)
}

/// Headers matcher.
#[derive(Debug)]
pub struct HeadersMatcher(Headers);

impl Matcher for HeadersMatcher {
    fn name(&self) -> &str {
        "headers"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers.is_superset(&self.0)
    }
}

pub fn headers(headers: Headers) -> HeadersMatcher {
    HeadersMatcher(headers)
}

/// Headers exact matcher.
#[derive(Debug)]
pub struct HeadersExactMatcher(Headers);

impl Matcher for HeadersExactMatcher {
    fn name(&self) -> &str {
        "headers_exact"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers == self.0
    }
}

pub fn headers_exact(headers: Headers) -> HeadersExactMatcher {
    HeadersExactMatcher(headers)
}

/// Header matcher.
#[derive(Debug)]
pub struct HeaderMatcher((String, String));

impl Matcher for HeaderMatcher {
    fn name(&self) -> &str {
        "header"
    }
    fn matches(&self, req: &Request) -> bool {
        let (name, value) = &self.0;
        req.headers.contains(name, value)
    }
}

pub fn header(name: impl Into<String>, value: impl Into<String>) -> HeaderMatcher {
    HeaderMatcher((name.into(), value.into()))
}

/// Header exists matcher.
#[derive(Debug)]
pub struct HeaderExistsMatcher(String);

impl Matcher for HeaderExistsMatcher {
    fn name(&self) -> &str {
        "header_exists"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers.contains_name(&self.0)
    }
}

pub fn header_exists(name: String) -> HeaderExistsMatcher {
    HeaderExistsMatcher(name)
}
