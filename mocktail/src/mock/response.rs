use bytes::Bytes;
use http::HeaderMap;

use super::MockBody;
use crate::utils::prost::MessageExt;

/// A mock response.
#[derive(Default, Debug, Clone)]
pub struct MockResponse {
    pub code: http::StatusCode,
    pub headers: HeaderMap,
    pub body: MockBody,
    pub message: Option<String>,
}

impl MockResponse {
    pub fn empty() -> Self {
        Self {
            body: MockBody::Empty,
            ..Default::default()
        }
    }

    pub fn new(body: impl Into<Bytes>) -> Self {
        Self::full(body)
    }

    pub fn full(body: impl Into<Bytes>) -> Self {
        Self {
            body: MockBody::Full(body.into()),
            ..Default::default()
        }
    }

    pub fn stream(messages: impl IntoIterator<Item = impl Into<Bytes>>) -> Self {
        Self {
            body: MockBody::Stream(messages.into_iter().map(|msg| msg.into()).collect()),
            ..Default::default()
        }
    }

    pub fn json(body: impl serde::Serialize) -> Self {
        Self {
            body: MockBody::Full(serde_json::to_vec(&body).unwrap().into()),
            ..Default::default()
        }
    }

    pub fn json_stream(messages: impl IntoIterator<Item = impl serde::Serialize>) -> Self {
        Self {
            body: MockBody::Stream(
                messages
                    .into_iter()
                    .map(|msg| serde_json::to_vec(&msg).unwrap().into())
                    .collect(),
            ),
            ..Default::default()
        }
    }

    pub fn pb(body: impl prost::Message) -> Self {
        Self {
            body: MockBody::Full(body.to_bytes()),
            ..Default::default()
        }
    }

    pub fn pb_stream(messages: impl IntoIterator<Item = impl prost::Message>) -> Self {
        Self {
            body: MockBody::Stream(messages.into_iter().map(|msg| msg.to_bytes()).collect()),
            ..Default::default()
        }
    }

    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_code(mut self, code: http::StatusCode) -> Self {
        self.code = code;
        self
    }

    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn code(&self) -> http::StatusCode {
        self.code
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &MockBody {
        &self.body
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn is_error(&self) -> bool {
        !matches!(self.code(), http::StatusCode::OK)
    }
}
