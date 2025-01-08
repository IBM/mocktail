use http::Method;

/// A mock path for request matching.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MockPath {
    method: Method,
    path: String,
    // TODO: probably add query strings here
    // query: Option<String> or path_and_query: http::uri::PathAndQuery,
}

impl MockPath {
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        let mut path: String = path.into();
        if !path.starts_with('/') {
            path = format!("/{path}");
        }
        Self { method, path }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl From<(Method, String)> for MockPath {
    fn from(value: (Method, String)) -> Self {
        Self {
            method: value.0,
            path: value.1,
        }
    }
}
