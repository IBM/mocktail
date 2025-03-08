use bytes::Bytes;
use futures::stream;
use http_body::Frame;
use http_body_util::{BodyExt, Empty, Full, StreamBody};

use crate::utils::prost::MessageExt;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

/// A mock body.
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Bytes(Bytes),
    Stream(Vec<Bytes>),
}

impl Body {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn raw(body: Vec<u8>) -> Self {
        Self::Bytes(body.into())
    }

    pub fn json(body: impl serde::Serialize) -> Self {
        Self::Bytes(serde_json::to_vec(&body).unwrap().into())
    }

    // TODO: change to ndjson_stream
    pub fn json_stream(messages: impl IntoIterator<Item = impl serde::Serialize>) -> Self {
        let bufs = messages
            .into_iter()
            .map(|msg| serde_json::to_vec(&msg).unwrap().into())
            .collect();
        Self::Stream(bufs)
    }

    pub fn pb(body: impl prost::Message) -> Self {
        Self::Bytes(body.to_bytes())
    }

    pub fn pb_stream(messages: impl IntoIterator<Item = impl prost::Message>) -> Self {
        Self::Stream(messages.into_iter().map(|msg| msg.to_bytes()).collect())
    }

    // TODO:
    // pub fn sse_stream() -> Self {}

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            Body::Bytes(chunk) => chunk.len(),
            Body::Stream(chunks) => chunks.iter().map(|chunk| chunk.len()).sum(),
        }
    }

    pub fn chunks(&self) -> Vec<Bytes> {
        match self {
            Body::Bytes(chunk) => vec![chunk.clone()],
            Body::Stream(chunks) => chunks.clone(),
        }
    }

    /// Returns a type-erased HTTP body for hyper.
    pub fn to_hyper_boxed(&self) -> BoxBody {
        if self.is_empty() {
            return Empty::new().map_err(|err| match err {}).boxed();
        }
        match self {
            Body::Bytes(chunk) => Full::new(chunk.clone())
                .map_err(|never| match never {})
                .boxed(),
            Body::Stream(chunks) => {
                let messages: Vec<Result<_, hyper::Error>> = chunks
                    .iter()
                    .map(|chunk| Ok(Frame::data(chunk.clone())))
                    .collect();
                BoxBody::new(StreamBody::new(stream::iter(messages)))
            }
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Body::Bytes(Bytes::new())
    }
}

impl PartialEq<[u8]> for Body {
    fn eq(&self, other: &[u8]) -> bool {
        match self {
            Body::Bytes(bytes) => bytes == other,
            Body::Stream(bufs) => bufs.concat() == other,
        }
    }
}

impl From<Bytes> for Body {
    fn from(value: Bytes) -> Self {
        Self::Bytes(value)
    }
}

impl From<Vec<Bytes>> for Body {
    fn from(value: Vec<Bytes>) -> Self {
        Self::Stream(value)
    }
}
