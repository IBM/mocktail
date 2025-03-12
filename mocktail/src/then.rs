use std::{cell::Cell, rc::Rc};

use bytes::Bytes;

use crate::{
    body::Body,
    headers::{HeaderName, HeaderValue, Headers},
    response::{Response, StatusCode},
};

/// A response builder.
#[derive(Default, Clone)]
pub struct Then(Rc<Cell<Response>>);

impl Then {
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(Response::default())))
    }

    /// Returns the inner response.
    pub fn into_inner(self) -> Response {
        self.0.take()
    }

    /// Updates the response.
    fn update<F: FnOnce(&mut Response)>(&self, f: F) {
        let mut r = self.0.take();
        f(&mut r);
        self.0.set(r);
    }

    /// HTTP status code.
    pub fn status(self, status: impl Into<StatusCode>) -> Self {
        self.update(|r| {
            r.status = status.into();
        });
        self
    }

    /// HTTP headers.
    pub fn headers<T, U>(self, headers: impl IntoIterator<Item = (T, U)>) -> Self
    where
        T: Into<HeaderName>,
        U: Into<HeaderValue>,
    {
        self.update(|r| {
            r.headers = Headers::from_iter(headers);
        });
        self
    }

    /// Body.
    pub fn body(self, body: Body) -> Self {
        self.update(|r| {
            r.body = body;
        });
        self
    }

    /// Error message.
    pub fn message(self, message: impl Into<String>) -> Self {
        self.update(|r| {
            r.message = Some(message.into());
        });
        self
    }
}

/// Body convenience methods.
impl Then {
    /// Empty body.
    pub fn empty(self) -> Self {
        self.update(|r| {
            r.body = Body::empty();
        });
        self
    }

    /// Raw bytes body.
    pub fn bytes(self, body: Vec<u8>) -> Self {
        self.update(|r| {
            r.body = Body::bytes(body);
        });
        self
    }

    /// Raw bytes streaming body.
    pub fn bytes_stream(self, messages: impl IntoIterator<Item = impl Into<Bytes>>) -> Self {
        self.update(|r| {
            r.body = Body::bytes_stream(messages);
        });
        self
    }

    /// Text body.
    pub fn text(self, body: impl Into<String>) -> Self {
        let body: String = body.into();
        self.update(|r| {
            r.body = Body::bytes(body);
        });
        self
    }

    /// Text streaming body.
    pub fn text_stream(self, messages: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let messages = messages.into_iter().map(|msg| {
            let msg: String = msg.into();
            msg
        });
        self.update(|r| {
            r.body = Body::bytes_stream(messages);
        });
        self
    }

    /// Json body.
    pub fn json(self, body: impl serde::Serialize) -> Self {
        self.update(|r| {
            r.headers.insert("content-type", "application/json");
            r.body = Body::json(body);
        });
        self
    }

    /// Newline delimited JSON streaming body.
    pub fn json_lines_stream(
        self,
        messages: impl IntoIterator<Item = impl serde::Serialize>,
    ) -> Self {
        self.update(|r| {
            r.headers.insert("content-type", "application/x-ndjson");
            r.body = Body::json_lines_stream(messages);
        });
        self
    }

    /// Protobuf body.
    pub fn pb(self, body: impl prost::Message) -> Self {
        self.update(|r| {
            r.body = Body::pb(body);
        });
        self
    }

    /// Protobuf streaming body.
    pub fn pb_stream(self, messages: impl IntoIterator<Item = impl prost::Message>) -> Self {
        self.update(|r| {
            r.body = Body::pb_stream(messages);
        });
        self
    }
}

/// Status convenience methods.
impl Then {
    /// Error status code and message.
    pub fn error(self, status: StatusCode, message: impl Into<String>) -> Self {
        self.update(|r| {
            r.status = status;
            r.message = Some(message.into());
        });
        self
    }

    pub fn ok(self) -> Self {
        self.update(|r| {
            r.status = StatusCode::OK;
        });
        self
    }

    pub fn not_found(self) -> Self {
        self.update(|r| {
            r.status = StatusCode::NOT_FOUND;
        });
        self
    }

    pub fn bad_request(self) -> Self {
        self.update(|r| {
            r.status = StatusCode::BAD_REQUEST;
        });
        self
    }

    pub fn internal_server_error(self) -> Self {
        self.update(|r| {
            r.status = StatusCode::INTERNAL_SERVER_ERROR;
        });
        self
    }
}
