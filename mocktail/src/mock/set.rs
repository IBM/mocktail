use std::collections::{hash_map, HashMap};

use super::{Mock, MockPath};

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

    /// Matches a [`Mock`] by method and request body.
    pub fn find(&self, path: &MockPath, body: &[u8]) -> Option<&Mock> {
        self.0
            .get(path)
            .and_then(|mocks| mocks.iter().find(|&mock| mock.request.body() == body))
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
