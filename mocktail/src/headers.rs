//! Headers
/// Represents HTTP headers.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Headers(Vec<(HeaderName, HeaderValue)>);

impl Headers {
    /// Creates an empty headers.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of headers.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Inserts a header.
    pub fn insert(&mut self, name: impl Into<HeaderName>, value: impl Into<HeaderValue>) {
        let name: HeaderName = name.into();
        let value: HeaderValue = value.into();
        if !self.contains(&name, &value) {
            self.0.push((name, value));
        }
    }

    /// Gets a header by name.
    pub fn get(&self, name: &str) -> Option<&HeaderValue> {
        self.0
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value)
    }

    /// Removes a header by name.
    pub fn remove(&mut self, name: &str) {
        self.0.retain(|(key, _)| key != name)
    }

    /// Clears the headers.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns true if the headers contains a header with this name.
    pub fn contains_name(&self, name: &str) -> bool {
        self.0.iter().any(|(key, _)| key == name)
    }

    /// Returns true if the headers contains the header.
    pub fn contains(&self, name: &str, value: &str) -> bool {
        self.0
            .iter()
            .any(|header| *header.0 == name && *header.1 == value)
    }

    /// Returns true if the headers are a subset of another,
    /// i.e., other contains at least all the values in self.
    pub fn is_subset(&self, other: &Headers) -> bool {
        self.0.iter().all(|header| other.0.contains(header))
    }

    /// Returns true if the headers are a superset of another,
    /// i.e., self contains at least all the values in other.
    pub fn is_superset(&self, other: &Headers) -> bool {
        other.is_subset(self)
    }

    /// Returns an iterator over the headers.
    pub fn iter(&self) -> std::slice::Iter<'_, (HeaderName, HeaderValue)> {
        self.0.iter()
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, HeaderValue);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, U> FromIterator<(T, U)> for Headers
where
    T: Into<HeaderName>,
    U: Into<HeaderValue>,
{
    fn from_iter<I: IntoIterator<Item = (T, U)>>(iter: I) -> Self {
        Self(
            iter.into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl From<Headers> for http::HeaderMap {
    fn from(value: Headers) -> Self {
        value
            .0
            .into_iter()
            .map(|(name, value)| (name.into(), value.into()))
            .collect()
    }
}

impl From<http::HeaderMap> for Headers {
    fn from(value: http::HeaderMap) -> Self {
        Self(
            value
                .into_iter()
                .map(|(name, value)| (name.unwrap().into(), value.into()))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<&http::HeaderMap> for Headers {
    fn from(value: &http::HeaderMap) -> Self {
        Self(
            value
                .iter()
                .map(|(name, value)| (name.into(), value.into()))
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Headers> for tonic::metadata::MetadataMap {
    fn from(value: Headers) -> Self {
        tonic::metadata::MetadataMap::from_headers(value.into())
    }
}

/// Represents a HTTP header name.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderName(String);

impl std::ops::Deref for HeaderName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<String> for HeaderName {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for HeaderName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl From<String> for HeaderName {
    fn from(value: String) -> Self {
        HeaderName(value)
    }
}

impl From<&str> for HeaderName {
    fn from(value: &str) -> Self {
        HeaderName(value.into())
    }
}

impl AsRef<str> for HeaderName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<HeaderName> for http::HeaderName {
    fn from(value: HeaderName) -> Self {
        http::HeaderName::try_from(value.0).unwrap()
    }
}

impl From<&HeaderName> for http::HeaderName {
    fn from(value: &HeaderName) -> Self {
        http::HeaderName::try_from(value.0.clone()).unwrap()
    }
}

impl From<http::HeaderName> for HeaderName {
    fn from(value: http::HeaderName) -> Self {
        HeaderName(value.to_string())
    }
}

impl From<&http::HeaderName> for HeaderName {
    fn from(value: &http::HeaderName) -> Self {
        HeaderName(value.to_string())
    }
}

/// Represents a HTTP header value.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderValue(String);

impl std::ops::Deref for HeaderValue {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq<String> for HeaderValue {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl PartialEq<str> for HeaderValue {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl From<String> for HeaderValue {
    fn from(value: String) -> Self {
        HeaderValue(value)
    }
}

impl From<&str> for HeaderValue {
    fn from(value: &str) -> Self {
        HeaderValue(value.into())
    }
}

impl AsRef<str> for HeaderValue {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<HeaderValue> for http::HeaderValue {
    fn from(value: HeaderValue) -> Self {
        http::HeaderValue::try_from(value.0).unwrap()
    }
}

impl From<&HeaderValue> for http::HeaderValue {
    fn from(value: &HeaderValue) -> Self {
        http::HeaderValue::try_from(value.0.clone()).unwrap()
    }
}

impl From<http::HeaderValue> for HeaderValue {
    fn from(value: http::HeaderValue) -> Self {
        HeaderValue(value.to_str().unwrap().to_string())
    }
}

impl From<&http::HeaderValue> for HeaderValue {
    fn from(value: &http::HeaderValue) -> Self {
        HeaderValue(value.to_str().unwrap().to_string())
    }
}
