use super::{body::ToBytes, Headers, MockBody};

/// A mock request.
#[derive(Default, Debug, Clone)]
pub struct MockRequest {
    pub headers: Headers,
    pub body: MockBody,
}

impl MockRequest {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new<T>(body: impl ToBytes<T>) -> Self {
        Self::full(body)
    }

    pub fn full<T>(body: impl ToBytes<T>) -> Self {
        Self {
            body: MockBody::Full(body.to_bytes()),
            ..Default::default()
        }
    }

    pub fn stream<T>(messages: impl IntoIterator<Item = impl ToBytes<T>>) -> Self {
        Self {
            body: MockBody::Stream(
                messages
                    .into_iter()
                    .map(|message| message.to_bytes())
                    .collect(),
            ),
            ..Default::default()
        }
    }

    pub fn with_headers(
        mut self,
        headers: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        self.headers = Headers::new(headers);
        self
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &MockBody {
        &self.body
    }
}
