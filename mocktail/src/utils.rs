use std::net::TcpListener;

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
                http::StatusCode::BAD_REQUEST | http::StatusCode::UNPROCESSABLE_ENTITY => {
                    tonic::Code::Internal
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

pub fn find_available_port() -> Option<u16> {
    let mut rng = rand::thread_rng();
    loop {
        let port: u16 = rng.gen_range(40000..60000);
        if port_is_available(port) {
            return Some(port);
        }
    }
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_ok()
}
