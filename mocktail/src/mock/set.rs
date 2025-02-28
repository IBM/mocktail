use std::collections::{hash_map, HashMap};

use super::Mock;
use crate::utils::HeaderMapExt;

/// A set of mocks for a service.
#[derive(Default, Debug, Clone)]
pub struct MockSet(HashMap<MockPath, Vec<Mock>>);

impl MockSet {
    /// Creates a empty [`MockSet`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a [`Mock`].
    pub fn insert(&mut self, path: MockPath, mock: Mock) {
        match self.0.entry(path) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(mock);
            }
            hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![mock]);
            }
        }
    }

    /// Matches a [`Mock`] by path and predicate.
    pub fn find<P>(&self, path: &MockPath, predicate: P) -> Option<&Mock>
    where
        P: FnMut(&&Mock) -> bool,
    {
        self.0
            .get(path)
            .and_then(|mocks| mocks.iter().find(predicate))
    }

    /// Matches a [`Mock`] by path and body.
    pub fn match_by_body(&self, path: &MockPath, body: &[u8]) -> Option<&Mock> {
        self.find(path, |mock| mock.request.body() == body)
    }

    /// Matches a [`Mock`] by path, body, and headers.
    pub fn match_by_body_and_headers(
        &self,
        path: &MockPath,
        body: &[u8],
        headers: &http::HeaderMap,
    ) -> Option<&Mock> {
        // `headers` must be a superset of `mock.request.headers`,
        if let Some(mock) = self.match_by_body(path, body) {
            if headers.is_superset(&mock.request.headers) {
                return Some(mock);
            }
        }
        None
    }
}

impl FromIterator<(MockPath, Vec<Mock>)> for MockSet {
    fn from_iter<T: IntoIterator<Item = (MockPath, Vec<Mock>)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl std::ops::Deref for MockSet {
    type Target = HashMap<MockPath, Vec<Mock>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MockPath {
    method: http::Method,
    path: String,
    query: Option<String>,
}

impl MockPath {
    pub fn new(method: &http::Method, path: &str) -> Self {
        let mut path: String = path.into();
        if !path.starts_with('/') {
            path = format!("/{path}");
        }
        Self {
            method: method.clone(),
            path,
            query: None,
        }
    }

    pub fn from_request<B>(request: &http::Request<B>) -> Self {
        request.into()
    }

    pub fn post(path: &str) -> Self {
        Self {
            method: http::Method::POST,
            path: path.into(),
            query: None,
        }
    }

    pub fn get(path: &str) -> Self {
        Self {
            method: http::Method::GET,
            path: path.into(),
            query: None,
        }
    }

    pub fn with_query(mut self, query: &str) -> Self {
        self.query = Some(query.into());
        self
    }

    pub fn method(&self) -> &http::Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }
}

impl<B> From<&http::Request<B>> for MockPath {
    fn from(value: &http::Request<B>) -> Self {
        let mut key = Self::new(value.method(), value.uri().path());
        if let Some(query) = value.uri().query() {
            key = key.with_query(query);
        }
        key
    }
}
