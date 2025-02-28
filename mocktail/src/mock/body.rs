use bytes::{BufMut, Bytes, BytesMut};
use futures::stream;
use http_body::Frame;
use http_body_util::{BodyExt, Full, StreamBody};

pub type HyperBoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;
pub type TonicBoxBody = http_body_util::combinators::UnsyncBoxBody<Bytes, tonic::Status>;

/// A mock body.
#[derive(Debug, Clone)]
pub enum MockBody {
    Full(Bytes),
    Stream(Vec<Bytes>),
}

impl MockBody {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        match self {
            MockBody::Full(chunk) => chunk.len(),
            MockBody::Stream(chunks) => chunks.iter().map(|chunk| chunk.len()).sum(),
        }
    }

    pub fn chunks(&self) -> Vec<Bytes> {
        match self {
            MockBody::Full(chunk) => vec![chunk.clone()],
            MockBody::Stream(chunks) => chunks.clone(),
        }
    }

    /// Returns a type-erased HTTP body for hyper.
    pub fn to_hyper_boxed(&self) -> HyperBoxBody {
        match self {
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

impl Default for MockBody {
    fn default() -> Self {
        Self::Full(Bytes::default())
    }
}

impl PartialEq<[u8]> for MockBody {
    fn eq(&self, other: &[u8]) -> bool {
        match self {
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

/// Converts payload to [`Bytes`].
pub trait ToBytes<Marker> {
    fn to_bytes(&self) -> Bytes;
}

// Ugly workaround using marker structs to enable multiple blanket impls.

pub struct Serializable;
impl<T> ToBytes<Serializable> for T
where
    T: serde::Serialize,
{
    fn to_bytes(&self) -> Bytes {
        serde_json::to_vec(self).unwrap().into()
    }
}
pub struct Protobuf;
impl<T> ToBytes<Protobuf> for T
where
    T: prost::Message,
{
    fn to_bytes(&self) -> Bytes {
        let mut buf = BytesMut::with_capacity(256);
        buf.reserve(5);
        unsafe {
            buf.advance_mut(5);
        }
        self.encode(&mut buf).unwrap();
        {
            let len = buf.len() - 5;
            let mut buf = &mut buf[..5];
            buf.put_u8(0); // byte must be 0
            buf.put_u32(len as u32);
        }
        buf.freeze()
    }
}
