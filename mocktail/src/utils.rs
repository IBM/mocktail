use std::net::TcpListener;

use http::{header, HeaderMap, HeaderName, HeaderValue};
use rand::Rng;

pub mod tonic {
    use http::status::InvalidStatusCode;

    pub trait CodeExt {
        /// Creates a gRPC status code from an equivalent HTTP status code.
        fn from_u16(code: u16) -> Result<tonic::Code, InvalidStatusCode>;
        /// Creates a gRPC status code from an equivalent [`http::StatusCode`].
        fn from_http(status_code: http::StatusCode) -> tonic::Code;
    }

    impl CodeExt for tonic::Code {
        fn from_u16(code: u16) -> Result<tonic::Code, InvalidStatusCode> {
            let status_code = http::StatusCode::from_u16(code)?;
            Ok(tonic::Code::from_http(status_code))
        }

        fn from_http(status_code: http::StatusCode) -> tonic::Code {
            match status_code {
                http::StatusCode::INTERNAL_SERVER_ERROR => tonic::Code::Internal,
                http::StatusCode::UNPROCESSABLE_ENTITY | http::StatusCode::BAD_REQUEST => {
                    tonic::Code::InvalidArgument
                }
                http::StatusCode::UNAUTHORIZED => tonic::Code::Unauthenticated,
                http::StatusCode::FORBIDDEN => tonic::Code::PermissionDenied,
                http::StatusCode::NOT_FOUND => tonic::Code::NotFound,
                http::StatusCode::NOT_IMPLEMENTED => tonic::Code::Unimplemented,
                http::StatusCode::TOO_MANY_REQUESTS
                | http::StatusCode::BAD_GATEWAY
                | http::StatusCode::SERVICE_UNAVAILABLE
                | http::StatusCode::GATEWAY_TIMEOUT => tonic::Code::Unavailable,
                http::StatusCode::OK => tonic::Code::Ok,
                _ => tonic::Code::Unknown,
            }
        }
    }
}

pub mod prost {
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
}

pub trait HeaderMapExt<T = HeaderValue> {
    /// Returns `true` if the map contains a key-value pair.
    fn contains(&self, key: &HeaderName, value: &HeaderValue) -> bool;

    /// Returns `true` if the map is a subset of another,
    /// i.e., `other` contains at least all the values in `self`.
    fn is_subset(&self, other: &HeaderMap<T>) -> bool;

    /// Returns `true` if the map is a superset of another,
    /// i.e., `self` contains at least all the values in `other`.
    fn is_superset(&self, other: &HeaderMap<T>) -> bool;
}

impl HeaderMapExt<HeaderValue> for HeaderMap<HeaderValue> {
    fn contains(&self, key: &HeaderName, value: &HeaderValue) -> bool {
        self.iter().any(|entry| entry.0 == key && entry.1 == value)
    }

    fn is_subset(&self, other: &HeaderMap<HeaderValue>) -> bool {
        self.iter().all(|(key, value)| other.contains(key, value))
    }

    fn is_superset(&self, other: &HeaderMap<HeaderValue>) -> bool {
        other.is_subset(self)
    }
}

pub fn find_available_port() -> Option<u16> {
    let mut rng = rand::rng();
    loop {
        let port: u16 = rng.random_range(40000..60000);
        if port_is_available(port) {
            return Some(port);
        }
    }
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}

pub fn has_content_type(headers: &HeaderMap, content_type: &str) -> bool {
    let header = headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap());
    header.is_some_and(|value| value == content_type)
}
