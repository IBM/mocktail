use std::collections::{hash_map, HashMap};

use super::Mock;

/// A set of mocks for a service.
#[derive(Default, Debug, Clone)]
pub struct MockSet(HashMap<MockPath, Vec<Mock>>);

impl MockSet {
    /// Creates an empty mockset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of entries in the mockset.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the mockset contains no entries.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

    /// Gets an entry for in-place manipulation.
    pub fn entry(&mut self, path: MockPath) -> hash_map::Entry<'_, MockPath, Vec<Mock>> {
        self.0.entry(path)
    }

    /// Removes an entry from the mockset.
    pub fn remove(&mut self, path: &MockPath) -> Option<Vec<Mock>> {
        self.0.remove(path)
    }

    /// Clears the mockset.
    pub fn clear(&mut self) {
        self.0.clear()
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
