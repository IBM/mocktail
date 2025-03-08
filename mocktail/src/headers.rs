/// A representation of HTTP headers.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headers(Vec<(String, String)>);

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
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<String>) {
        let name: String = name.into();
        let value: String = value.into();
        if !self.contains(&name, &value) {
            self.0.push((name, value));
        }
    }

    /// Gets a header by name.
    pub fn get(&self, name: &str) -> Option<&(String, String)> {
        self.0.iter().find(|(key, _)| key == name)
    }

    /// Removes a header by name.
    pub fn remove(&mut self, name: &str) {
        self.0.retain(|(key, _)| key != name)
    }

    /// Clears the headers.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns `true` if the headers contains a header with this name.
    pub fn contains_name(&self, name: &str) -> bool {
        self.0.iter().any(|(key, _)| key == name)
    }

    /// Returns `true` if the headers contains the header.
    pub fn contains(&self, name: &str, value: &str) -> bool {
        self.0
            .iter()
            .any(|header| header.0 == name && header.1 == value)
    }

    /// Returns `true` if the headers are a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    pub fn is_subset(&self, other: &Headers) -> bool {
        self.0.iter().all(|header| other.0.contains(header))
    }

    /// Returns `true` if the headers are a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    pub fn is_superset(&self, other: &Headers) -> bool {
        other.is_subset(self)
    }

    /// Returns an iterator over the headers.
    pub fn iter(&self) -> std::slice::Iter<'_, (String, String)> {
        self.0.iter()
    }
}

impl IntoIterator for Headers {
    type Item = (String, String);

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> FromIterator<(T, T)> for Headers
where
    T: Into<String>,
{
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
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
            .iter()
            .map(|(key, value)| {
                let name = http::HeaderName::try_from(key).unwrap();
                let value = http::HeaderValue::try_from(value).unwrap();
                (name, value)
            })
            .collect()
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

impl From<Headers> for tonic::metadata::MetadataMap {
    fn from(value: Headers) -> Self {
        tonic::metadata::MetadataMap::from_headers(value.into())
    }
}
