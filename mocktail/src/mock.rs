//! Mock
use std::sync::Arc;

use uuid::Uuid;

use crate::{
    matchers::Matcher,
    mock_builder::{Then, When},
    request::Request,
    response::Response,
};

const DEFAULT_PRIORITY: u8 = 5;

/// A mock.
#[derive(Debug, Clone, PartialEq)]
pub struct Mock {
    /// Mock ID.
    pub id: Uuid,
    /// A set of request match conditions.
    pub matchers: Vec<Arc<dyn Matcher>>,
    /// A mock response.
    pub response: Response,
    /// Priority.
    pub priority: u8,
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
        }
    }

    /// Sets the mock priority.
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority;
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

    /// Evaluates a request against match conditions.
    pub fn matches(&self, req: &Request) -> bool {
        self.matchers.iter().all(|matcher| matcher.matches(req))
    }
}
