use std::{collections::vec_deque, convert::Infallible, pin::Pin, task::Poll};

use bytes::{Buf, Bytes};
use futures::Stream;
use http_body::Frame;

use crate::{buf_list::BufList, ext::MessageExt};

/// A mock body.
#[derive(Default, Debug, Clone)]
pub struct Body {
    bufs: BufList,
}

impl Body {
    /// Creates an empty body.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a bytes body.
    pub fn raw(body: impl Into<Bytes>) -> Self {
        let bytes: Bytes = body.into();
        Self { bufs: bytes.into() }
    }

    /// Creates a JSON body.
    pub fn json(body: impl serde::Serialize) -> Self {
        let bytes = serde_json::to_vec(&body).unwrap();
        Self { bufs: bytes.into() }
    }

    /// Creates a newline delimited JSON streaming body.
    pub fn json_lines_stream(messages: impl IntoIterator<Item = impl serde::Serialize>) -> Self {
        let bufs = messages
            .into_iter()
            .map(|msg| {
                let mut bytes = serde_json::to_vec(&msg).unwrap();
                bytes.push(b'\n');
                bytes.into()
            })
            .collect();
        Self { bufs }
    }

    /// Creates a protobuf body.
    pub fn pb(body: impl prost::Message) -> Self {
        let bytes = body.to_bytes();
        Self { bufs: bytes.into() }
    }

    /// Creates a protobuf streaming body.
    pub fn pb_stream(messages: impl IntoIterator<Item = impl prost::Message>) -> Self {
        let bufs = messages.into_iter().map(|msg| msg.to_bytes()).collect();
        Self { bufs }
    }

    // TODO
    // pub fn sse_stream() -> Self {}

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the byte length of the body.
    pub fn len(&self) -> usize {
        self.bufs.remaining()
    }

    pub fn as_bytes(&mut self) -> Bytes {
        self.bufs.as_bytes()
    }

    pub fn iter(&self) -> vec_deque::Iter<'_, Bytes> {
        self.bufs.iter()
    }
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        // We want to compare the merged bytes from all bufs
        // as the request body will be buffered chunks.
        // TODO: figure out a better approach with less overhead?
        self.bufs.clone().as_bytes() == other.bufs.clone().as_bytes()
    }
}

impl Stream for Body {
    type Item = Bytes;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if let Some(data) = self.bufs.pop() {
            Poll::Ready(Some(data))
        } else {
            Poll::Ready(None)
        }
    }
}

impl http_body::Body for Body {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        if let Some(data) = self.bufs.pop() {
            let frame = Frame::data(data);
            Poll::Ready(Some(Ok(frame)))
        } else {
            Poll::Ready(None)
        }
    }
}

impl From<Bytes> for Body {
    fn from(value: Bytes) -> Self {
        Self::raw(value)
    }
}
