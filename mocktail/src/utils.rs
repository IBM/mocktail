use http::{header, HeaderMap};

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

pub fn has_content_type(headers: &HeaderMap, content_type: &str) -> bool {
    let header = headers
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap());
    header.is_some_and(|value| value == content_type)
}
