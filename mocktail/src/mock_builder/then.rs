use std::{cell::Cell, rc::Rc};

use crate::{
    body::Body,
    headers::Headers,
    response::{Response, StatusCode},
};

/// A response builder.
#[derive(Default, Clone)]
pub struct Then(Rc<Cell<Response>>);

impl Then {
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(Response::default())))
    }

    pub fn into_inner(self) -> Response {
        self.0.take()
    }

    fn update<F: FnOnce(&mut Response)>(&self, f: F) {
        let mut r = self.0.take();
        f(&mut r);
        self.0.set(r);
    }

    /// HTTP status code.
    pub fn status(self, status: StatusCode) -> Self {
        self.update(|r| {
            r.status = status;
        });
        self
    }

    /// HTTP headers.
    pub fn headers<T>(self, headers: impl IntoIterator<Item = (T, T)>) -> Self
    where
        T: Into<String>,
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

    /// Raw bytes body.
    pub fn raw(self, body: Vec<u8>) -> Self {
        self.update(|r| {
            r.body = Body::raw(body);
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

    // TODO: change to ndjson_stream()
    pub fn json_stream(
        self,
        messages: impl IntoIterator<Item = impl serde::Serialize>,
    ) -> Self {
        self.update(|r| {
            r.body = Body::json_stream(messages);
        });
        self
    }

    /// Protobuf body.
    pub fn pb(self, body: impl prost::Message) -> Self {
        self.update(|r| {
            r.headers.insert("content-type", "application/grpc");
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

    /// Error message.
    pub fn message(self, message: impl Into<String>) -> Self {
        self.update(|r| {
            r.message = Some(message.into());
        });
        self
    }

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
