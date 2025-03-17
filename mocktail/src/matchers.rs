//! Mock request matchers
use super::{body::Body, headers::Headers, request::Request};
use crate::request::Method;
use std::{any::Any, borrow::Cow, cmp::Ordering};

/// A matcher.
pub trait Matcher: std::fmt::Debug + Send + Sync + 'static {
    /// Matcher name.
    fn name(&self) -> &str;
    /// Evaluates a match condition.
    fn matches(&self, req: &Request) -> bool;
    /// Returns matcher as [`&dyn MatcherEq`] to compare to another matcher.
    /// This is a workaround for dyn-compatability.
    #[doc(hidden)]
    fn as_matcher_eq(&self) -> &dyn MatcherEq;
}

/// Any matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct AnyMatcher;

impl Matcher for AnyMatcher {
    fn name(&self) -> &str {
        "any"
    }
    fn matches(&self, _req: &Request) -> bool {
        true
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn any() -> AnyMatcher {
    AnyMatcher
}

/// HTTP method matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct MethodMatcher(Method);

impl Matcher for MethodMatcher {
    fn name(&self) -> &str {
        "method"
    }
    fn matches(&self, req: &Request) -> bool {
        req.method == self.0
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn method(method: Method) -> MethodMatcher {
    MethodMatcher(method)
}

/// Path matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct PathMatcher(String);

impl Matcher for PathMatcher {
    fn name(&self) -> &str {
        "path"
    }
    fn matches(&self, req: &Request) -> bool {
        req.path() == self.0
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn path(path: impl Into<String>) -> PathMatcher {
    PathMatcher(path.into())
}

/// Path prefix matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct PathPrefixMatcher(String);

impl Matcher for PathPrefixMatcher {
    fn name(&self) -> &str {
        "path_prefix"
    }
    fn matches(&self, req: &Request) -> bool {
        req.path().starts_with(&self.0)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn path_prefix(prefix: impl Into<String>) -> PathPrefixMatcher {
    PathPrefixMatcher(prefix.into())
}

/// Body matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct BodyMatcher(Body);

impl Matcher for BodyMatcher {
    fn name(&self) -> &str {
        "body"
    }
    fn matches(&self, req: &Request) -> bool {
        self.0 == req.body
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn body(body: Body) -> BodyMatcher {
    BodyMatcher(body)
}

/// Headers matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct HeadersMatcher(Headers);

impl Matcher for HeadersMatcher {
    fn name(&self) -> &str {
        "headers"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers.is_superset(&self.0)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn headers(headers: Headers) -> HeadersMatcher {
    HeadersMatcher(headers)
}

/// Headers exact matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct HeadersExactMatcher(Headers);

impl Matcher for HeadersExactMatcher {
    fn name(&self) -> &str {
        "headers_exact"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers == self.0
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn headers_exact(headers: Headers) -> HeadersExactMatcher {
    HeadersExactMatcher(headers)
}

/// Header matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct HeaderMatcher(String, String);

impl Matcher for HeaderMatcher {
    fn name(&self) -> &str {
        "header"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers.contains(&self.0, &self.1)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn header(name: impl Into<String>, value: impl Into<String>) -> HeaderMatcher {
    HeaderMatcher(name.into(), value.into())
}

/// Header exists matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct HeaderExistsMatcher(String);

impl Matcher for HeaderExistsMatcher {
    fn name(&self) -> &str {
        "header_exists"
    }
    fn matches(&self, req: &Request) -> bool {
        req.headers.contains_name(&self.0)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn header_exists(name: impl Into<String>) -> HeaderExistsMatcher {
    HeaderExistsMatcher(name.into())
}

/// Query params matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct QueryParamsMatcher(Vec<(Cow<'static, str>, Cow<'static, str>)>);

impl Matcher for QueryParamsMatcher {
    fn name(&self) -> &str {
        "query_params"
    }
    fn matches(&self, req: &Request) -> bool {
        let pairs = req.query_pairs().collect::<Vec<_>>();
        pairs == self.0
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn query_params(
    pairs: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> QueryParamsMatcher {
    let pairs = pairs
        .into_iter()
        .map(|(key, value)| (Cow::from(key.into()), Cow::from(value.into())))
        .collect::<Vec<_>>();
    QueryParamsMatcher(pairs)
}

/// Query param matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct QueryParamMatcher(String, String);

impl Matcher for QueryParamMatcher {
    fn name(&self) -> &str {
        "query_param"
    }
    fn matches(&self, req: &Request) -> bool {
        req.query_pairs()
            .any(|(key, value)| key == self.0 && value == self.1)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn query_param(key: impl Into<String>, value: impl Into<String>) -> QueryParamMatcher {
    QueryParamMatcher(key.into(), value.into())
}

/// Query param exists matcher.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct QueryParamExistsMatcher(String);

impl Matcher for QueryParamExistsMatcher {
    fn name(&self) -> &str {
        "query_param_exists"
    }
    fn matches(&self, req: &Request) -> bool {
        req.query_pairs().any(|(key, _)| key == self.0)
    }
    fn as_matcher_eq(&self) -> &dyn MatcherEq {
        self
    }
}

pub fn query_param_exists(key: impl Into<String>) -> QueryParamExistsMatcher {
    QueryParamExistsMatcher(key.into())
}

#[doc(hidden)]
pub trait MatcherEq: Matcher {
    fn as_any(&self) -> &dyn Any;
    fn m_eq(&self, other: &dyn MatcherEq) -> bool;
    fn m_partial_cmp(&self, other: &dyn MatcherEq) -> Option<Ordering>;
}

/// Implements [`MatcherEq`] for all matchers.
impl<T: Matcher + PartialEq + PartialOrd> MatcherEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn m_eq(&self, other: &dyn MatcherEq) -> bool {
        other.as_any().downcast_ref::<Self>() == Some(self)
    }
    fn m_partial_cmp(&self, other: &dyn MatcherEq) -> Option<Ordering> {
        other
            .as_any()
            .downcast_ref::<Self>()
            .and_then(|other| self.partial_cmp(other))
    }
}

impl PartialEq<dyn MatcherEq> for dyn MatcherEq {
    fn eq(&self, other: &dyn MatcherEq) -> bool {
        self.m_eq(other)
    }
}

impl Eq for dyn MatcherEq {}

impl PartialOrd<dyn MatcherEq> for dyn MatcherEq {
    fn partial_cmp(&self, other: &dyn MatcherEq) -> Option<Ordering> {
        self.m_partial_cmp(other)
    }
}

impl PartialEq for dyn Matcher {
    fn eq(&self, other: &Self) -> bool {
        self.as_matcher_eq() == other.as_matcher_eq()
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
        let compare = self.as_matcher_eq().m_partial_cmp(other.as_matcher_eq());
        if let Some(ordering) = compare {
            ordering
        } else {
            self.name().cmp(other.name())
        }
    }
}
