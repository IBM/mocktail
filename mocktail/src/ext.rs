//! Extension traits
use bytes::{BufMut, Bytes, BytesMut};
use prost::Message;

pub trait MessageExt {
    /// Encodes the messages to bytes for a HTTP body.
    fn to_bytes(&self) -> Bytes;
}

impl<T: Message> MessageExt for T {
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
