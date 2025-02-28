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
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            MockBody::Empty => 0,
            MockBody::Full(chunk) => chunk.len(),
            MockBody::Stream(chunks) => chunks.iter().map(|chunk| chunk.len()).sum(),
        }
    }

    pub fn chunks(&self) -> Vec<Bytes> {
        match self {
            MockBody::Empty => vec![],
            MockBody::Full(chunk) => vec![chunk.clone()],
            MockBody::Stream(chunks) => chunks.clone(),
        }
    }

    /// Returns a type-erased HTTP body for hyper.
    pub fn to_hyper_boxed(&self) -> HyperBoxBody {
        match self {
            MockBody::Empty => Empty::new().map_err(|err| match err {}).boxed(),
            MockBody::Full(chunk) => Full::new(chunk.clone())
                .map_err(|never| match never {})
                .boxed(),
            MockBody::Stream(chunks) => {
                let messages: Vec<Result<_, hyper::Error>> = chunks
                    .iter()
                    .map(|chunk| Ok(Frame::data(chunk.clone())))
                    .collect();
                HyperBoxBody::new(StreamBody::new(stream::iter(messages)))
            }
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
