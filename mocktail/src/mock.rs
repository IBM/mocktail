//! Mock
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use uuid::Uuid;

use crate::{
    matchers::Matcher,
    mock_builder::{Then, When},
    request::Request,
    response::Response,
};

const DEFAULT_PRIORITY: u8 = 5;

/// A mock.
#[derive(Debug)]
pub struct Mock {
    /// Mock ID.
    pub id: Uuid,
    /// A set of request match conditions.
    pub matchers: Vec<Arc<dyn Matcher>>,
    /// A mock response.
    pub response: Response,
    /// Priority.
    pub priority: u8,
    /// Match counter.
    pub match_count: AtomicUsize,
    /// Limit on how many times this mock can be matched.
    pub limit: Option<usize>,
}

impl Mock {
    /// Builds a mock.
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(When, Then),
    {
        let id = Uuid::now_v7();
        let when = When::new();
        let then = Then::new();
        f(when.clone(), then.clone());
        Self {
            id,
            matchers: when.into_inner(),
            response: then.into_inner(),
            priority: DEFAULT_PRIORITY,
            match_count: AtomicUsize::new(0),
            limit: None,
        }
    }

    /// Sets the mock priority.
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Sets the mock limit.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Returns the mock ID.
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the mock response.
    pub fn response(&self) -> &Response {
        &self.response
    }

    /// Returns the mock priority.
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Returns the match count.
    pub fn match_count(&self) -> usize {
        self.match_count.load(Ordering::Relaxed)
    }

    /// Evaluates a request against match conditions.
    pub fn matches(&self, req: &Request) -> bool {
        if let Some(limit) = self.limit {
            if self.match_count.load(Ordering::Relaxed) >= limit {
                return false;
            }
        }
        let matched = self.matchers.iter().all(|matcher| matcher.matches(req));
        if matched {
            self.match_count.fetch_add(1, Ordering::Relaxed);
        }
        matched
    }

    /// Resets the match counter.
    pub fn reset(&self) {
        self.match_count.store(0, Ordering::Relaxed);
    }
}

impl PartialEq for Mock {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.matchers == other.matchers
            && self.response == other.response
            && self.priority == other.priority
            && self.match_count.load(Ordering::Relaxed) == other.match_count.load(Ordering::Relaxed)
            && self.limit == other.limit
    }
}

impl Clone for Mock {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            matchers: self.matchers.clone(),
            response: self.response.clone(),
            priority: self.priority,
            match_count: AtomicUsize::new(self.match_count.load(Ordering::Relaxed)),
            limit: self.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Method;

    #[test]
    fn test_match_counter() {
        let mock = Mock::new(|when, then| {
            when.get();
            then.ok();
        });
        let request = Request::new(Method::GET, "http://localhost/".parse().unwrap());
        mock.matches(&request);
        assert_eq!(mock.match_count(), 1);
        mock.matches(&request);
        assert_eq!(mock.match_count(), 2);
        mock.reset();
        assert_eq!(mock.match_count(), 0);
    }

    #[test]
    fn test_limit() {
        let mock = Mock::new(|when, then| {
            when.get();
            then.ok();
        })
        .with_limit(2);
        let request = Request::new(Method::GET, "http://localhost/".parse().unwrap());
        assert!(mock.matches(&request));
        assert!(mock.matches(&request));
        assert!(!mock.matches(&request));
    }
}
