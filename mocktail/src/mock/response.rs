use super::{body::ToBytes, Event, Headers, MockBody, StatusCode};

/// A mock response.
#[derive(Default, Debug, Clone)]
pub struct MockResponse {
    pub code: StatusCode,
    pub headers: Headers,
    pub body: MockBody,
    pub message: Option<String>,
}

impl MockResponse {
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

    pub fn sse_stream(messages: impl IntoIterator<Item = Event>) -> Self {
        Self {
            body: MockBody::Stream(messages.into_iter().map(|message| message.into()).collect()),
            ..Default::default()
        }
    }

    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_code(mut self, code: u16) -> Self {
        self.code = StatusCode::from_u16(code).unwrap();
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn with_error(mut self, code: u16, message: impl Into<String>) -> Self {
        self.code = StatusCode::from_u16(code).unwrap();
        self.message = Some(message.into());
        self
    }

    pub fn code(&self) -> StatusCode {
        self.code
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn body(&self) -> &MockBody {
        &self.body
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn is_success(&self) -> bool {
        self.code.is_success()
    }
}
