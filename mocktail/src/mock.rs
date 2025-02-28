mod body;
pub use body::{HyperBoxBody, MockBody, TonicBoxBody};

mod set;
pub use set::{MockPath, MockSet};

mod request;
pub use request::MockRequest;

mod response;
pub use response::MockResponse;

/// A mock request and response pair.
#[derive(Debug, Clone)]
pub struct Mock {
    pub request: MockRequest,
    pub response: MockResponse,
}

impl Mock {
    /// Creates a new [`Mock`].
    pub fn new(request: MockRequest, response: MockResponse) -> Self {
        Self { request, response }
    }
}
