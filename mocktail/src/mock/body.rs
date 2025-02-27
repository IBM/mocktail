use bytes::Bytes;
use futures::stream;
use http_body::Frame;
use http_body_util::{BodyExt, Empty, Full, StreamBody};

pub type HyperBoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
pub type TonicBoxBody = http_body_util::combinators::UnsyncBoxBody<Bytes, tonic::Status>;

/// A mock body.
#[derive(Default, Debug, Clone)]
pub enum MockBody {
    #[default]
    Empty,
    Full(Bytes),
    Stream(Vec<Bytes>),
}

impl MockBody {
    /// Returns a type-erased HTTP body for hyper.
    pub fn to_hyper_boxed(&self) -> HyperBoxBody {
        match self {
            MockBody::Empty => Empty::new().map_err(|err| match err {}).boxed(),
            MockBody::Full(data) => Full::new(data.clone())
                .map_err(|never| match never {})
                .boxed(),
            MockBody::Stream(data) => {
                let messages: Vec<Result<_, hyper::Error>> = data
                    .iter()
                    .map(|message| Ok(Frame::data(message.clone())))
                    .collect();
                HyperBoxBody::new(StreamBody::new(stream::iter(messages)))
            }
        }
    }

    /// Returns a type-erased HTTP body for tonic.
    pub fn to_tonic_boxed(&self) -> TonicBoxBody {
        match self {
            MockBody::Empty => tonic::body::empty_body(),
            MockBody::Full(data) => tonic::body::boxed(Full::new(data.clone())),
            MockBody::Stream(data) => {
                let messages: Vec<Result<_, tonic::Status>> = data
                    .iter()
                    .map(|message| Ok(Frame::data(message.clone())))
                    .collect();
                TonicBoxBody::new(StreamBody::new(stream::iter(messages)))
            }
        }
    }

    pub fn messages(&self) -> Vec<Bytes> {
        match self {
            MockBody::Empty => vec![],
            MockBody::Full(bytes) => vec![bytes.clone()],
            MockBody::Stream(items) => items.clone(),
        }
    }
}

impl PartialEq<[u8]> for MockBody {
    fn eq(&self, other: &[u8]) -> bool {
        match self {
            MockBody::Empty => other.is_empty(),
            MockBody::Full(bytes) => bytes == other,
            MockBody::Stream(data) => data.concat() == other,
        }
    }
}

impl From<Bytes> for MockBody {
    fn from(value: Bytes) -> Self {
        Self::Full(value)
    }
}

impl From<Vec<Bytes>> for MockBody {
    fn from(value: Vec<Bytes>) -> Self {
        Self::Stream(value)
    }
}
