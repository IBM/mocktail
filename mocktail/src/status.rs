use std::num::NonZeroU16;

use crate::Error;

/// Represents a HTTP status code.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

impl StatusCode {
    pub fn from_u16(code: u16) -> Result<Self, Error> {
        if !(100..1000).contains(&code) {
            return Err(Error::Invalid("invalid status code".into()));
        }
        Ok(Self(NonZeroU16::new(code).unwrap()))
    }

    pub fn as_u16(&self) -> u16 {
        self.0.get()
    }

    pub fn is_informational(&self) -> bool {
        (100..200).contains(&self.as_u16())
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.as_u16())
    }

    pub fn is_redirection(&self) -> bool {
        (300..400).contains(&self.as_u16())
    }

    pub fn is_error(&self) -> bool {
        (400..600).contains(&self.as_u16())
    }

    pub fn is_ok(&self) -> bool {
        self.is_success()
    }

    pub fn as_http(&self) -> http::StatusCode {
        http::StatusCode::from_u16(self.as_u16()).unwrap()
    }

    pub fn as_grpc(&self) -> Code {
        Code::from_http_u16(self.as_u16()).unwrap()
    }

    pub fn as_grpc_i32(&self) -> i32 {
        self.as_grpc() as i32
    }
}

impl StatusCode {
    pub const CONTINUE: StatusCode = StatusCode(NonZeroU16::new(100).unwrap());
    pub const SWITCHING_PROTOCOLS: StatusCode = StatusCode(NonZeroU16::new(101).unwrap());
    pub const PROCESSING: StatusCode = StatusCode(NonZeroU16::new(102).unwrap());
    pub const EARLY_HINTS: StatusCode = StatusCode(NonZeroU16::new(103).unwrap());

    pub const OK: StatusCode = StatusCode(NonZeroU16::new(200).unwrap());
    pub const CREATED: StatusCode = StatusCode(NonZeroU16::new(201).unwrap());
    pub const ACCEPTED: StatusCode = StatusCode(NonZeroU16::new(202).unwrap());
    pub const NON_AUTHORITATIVE_INFORMATION: StatusCode = StatusCode(NonZeroU16::new(203).unwrap());
    pub const NO_CONTENT: StatusCode = StatusCode(NonZeroU16::new(204).unwrap());
    pub const RESET_CONTENT: StatusCode = StatusCode(NonZeroU16::new(205).unwrap());
    pub const PARTIAL_CONTENT: StatusCode = StatusCode(NonZeroU16::new(206).unwrap());
    pub const MULTI_STATUS: StatusCode = StatusCode(NonZeroU16::new(207).unwrap());
    pub const ALREADY_REPORTED: StatusCode = StatusCode(NonZeroU16::new(208).unwrap());
    pub const IM_USED: StatusCode = StatusCode(NonZeroU16::new(226).unwrap());

    pub const MULTIPLE_CHOICES: StatusCode = StatusCode(NonZeroU16::new(300).unwrap());
    pub const MOVED_PERMANENTLY: StatusCode = StatusCode(NonZeroU16::new(301).unwrap());
    pub const FOUND: StatusCode = StatusCode(NonZeroU16::new(302).unwrap());
    pub const SEE_OTHER: StatusCode = StatusCode(NonZeroU16::new(303).unwrap());
    pub const NOT_MODIFIED: StatusCode = StatusCode(NonZeroU16::new(304).unwrap());
    pub const USE_PROXY: StatusCode = StatusCode(NonZeroU16::new(305).unwrap());
    pub const TEMPORARY_REDIRECT: StatusCode = StatusCode(NonZeroU16::new(307).unwrap());
    pub const PERMANENT_REDIRECT: StatusCode = StatusCode(NonZeroU16::new(308).unwrap());

    pub const BAD_REQUEST: StatusCode = StatusCode(NonZeroU16::new(400).unwrap());
    pub const UNAUTHORIZED: StatusCode = StatusCode(NonZeroU16::new(401).unwrap());
    pub const PAYMENT_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(402).unwrap());
    pub const FORBIDDEN: StatusCode = StatusCode(NonZeroU16::new(403).unwrap());
    pub const NOT_FOUND: StatusCode = StatusCode(NonZeroU16::new(404).unwrap());
    pub const METHOD_NOT_ALLOWED: StatusCode = StatusCode(NonZeroU16::new(405).unwrap());
    pub const NOT_ACCEPTABLE: StatusCode = StatusCode(NonZeroU16::new(406).unwrap());
    pub const PROXY_AUTHENTICATION_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(407).unwrap());
    pub const REQUEST_TIMEOUT: StatusCode = StatusCode(NonZeroU16::new(408).unwrap());
    pub const CONFLICT: StatusCode = StatusCode(NonZeroU16::new(409).unwrap());
    pub const GONE: StatusCode = StatusCode(NonZeroU16::new(410).unwrap());
    pub const LENGTH_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(411).unwrap());
    pub const PRECONDITION_FAILED: StatusCode = StatusCode(NonZeroU16::new(412).unwrap());
    pub const PAYLOAD_TOO_LARGE: StatusCode = StatusCode(NonZeroU16::new(413).unwrap());
    pub const URI_TOO_LONG: StatusCode = StatusCode(NonZeroU16::new(414).unwrap());
    pub const UNSUPPORTED_MEDIA_TYPE: StatusCode = StatusCode(NonZeroU16::new(415).unwrap());
    pub const RANGE_NOT_SATISFIABLE: StatusCode = StatusCode(NonZeroU16::new(416).unwrap());
    pub const EXPECTATION_FAILED: StatusCode = StatusCode(NonZeroU16::new(417).unwrap());
    pub const IM_A_TEAPOT: StatusCode = StatusCode(NonZeroU16::new(418).unwrap());
    pub const MISDIRECTED_REQUEST: StatusCode = StatusCode(NonZeroU16::new(421).unwrap());
    pub const UNPROCESSABLE_ENTITY: StatusCode = StatusCode(NonZeroU16::new(422).unwrap());
    pub const LOCKED: StatusCode = StatusCode(NonZeroU16::new(423).unwrap());
    pub const FAILED_DEPENDENCY: StatusCode = StatusCode(NonZeroU16::new(424).unwrap());
    pub const TOO_EARLY: StatusCode = StatusCode(NonZeroU16::new(425).unwrap());
    pub const UPGRADE_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(426).unwrap());
    pub const PRECONDITION_REQUIRED: StatusCode = StatusCode(NonZeroU16::new(428).unwrap());
    pub const TOO_MANY_REQUESTS: StatusCode = StatusCode(NonZeroU16::new(429).unwrap());
    pub const REQUEST_HEADER_FIELDS_TOO_LARGE: StatusCode =
        StatusCode(NonZeroU16::new(431).unwrap());
    pub const UNAVAILABLE_FOR_LEGAL_REASONS: StatusCode = StatusCode(NonZeroU16::new(451).unwrap());

    pub const INTERNAL_SERVER_ERROR: StatusCode = StatusCode(NonZeroU16::new(500).unwrap());
    pub const NOT_IMPLEMENTED: StatusCode = StatusCode(NonZeroU16::new(501).unwrap());
    pub const BAD_GATEWAY: StatusCode = StatusCode(NonZeroU16::new(502).unwrap());
    pub const SERVICE_UNAVAILABLE: StatusCode = StatusCode(NonZeroU16::new(503).unwrap());
    pub const GATEWAY_TIMEOUT: StatusCode = StatusCode(NonZeroU16::new(504).unwrap());
    pub const HTTP_VERSION_NOT_SUPPORTED: StatusCode = StatusCode(NonZeroU16::new(505).unwrap());
    pub const VARIANT_ALSO_NEGOTIATES: StatusCode = StatusCode(NonZeroU16::new(506).unwrap());
    pub const INSUFFICIENT_STORAGE: StatusCode = StatusCode(NonZeroU16::new(507).unwrap());
    pub const LOOP_DETECTED: StatusCode = StatusCode(NonZeroU16::new(508).unwrap());
    pub const NOT_EXTENDED: StatusCode = StatusCode(NonZeroU16::new(510).unwrap());
    pub const NETWORK_AUTHENTICATION_REQUIRED: StatusCode =
        StatusCode(NonZeroU16::new(511).unwrap());
}

impl Default for StatusCode {
    fn default() -> Self {
        Self::OK
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl PartialEq<StatusCode> for http::StatusCode {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value)
    }
}

impl From<StatusCode> for u16 {
    fn from(status: StatusCode) -> u16 {
        status.0.get()
    }
}

impl From<http::StatusCode> for StatusCode {
    fn from(value: http::StatusCode) -> Self {
        Self::from_u16(value.as_u16()).unwrap()
    }
}

impl From<StatusCode> for http::StatusCode {
    fn from(value: StatusCode) -> Self {
        Self::from_u16(value.0.into()).unwrap()
    }
}

impl From<StatusCode> for Code {
    fn from(value: StatusCode) -> Self {
        Code::from_http_u16(value.as_u16()).unwrap()
    }
}

/// Represents a gRPC status code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    /// The operation completed successfully.
    Ok = 0,
    /// The operation was cancelled.
    Cancelled = 1,
    /// Unknown error.
    Unknown = 2,
    /// Client specified an invalid argument.
    InvalidArgument = 3,
    /// Deadline expired before operation could complete.
    DeadlineExceeded = 4,
    /// Some requested entity was not found.
    NotFound = 5,
    /// Some entity that we attempted to create already exists.
    AlreadyExists = 6,
    /// The caller does not have permission to execute the specified operation.
    PermissionDenied = 7,
    /// Some resource has been exhausted.
    ResourceExhausted = 8,
    /// The system is not in a state required for the operation's execution.
    FailedPrecondition = 9,
    /// The operation was aborted.
    Aborted = 10,
    /// Operation was attempted past the valid range.
    OutOfRange = 11,
    /// Operation is not implemented or not supported.
    Unimplemented = 12,
    /// Internal error.
    Internal = 13,
    /// The service is currently unavailable.
    Unavailable = 14,
    /// Unrecoverable data loss or corruption.
    DataLoss = 15,
    /// The request does not have valid authentication credentials
    Unauthenticated = 16,
}

impl Code {
    /// Returns the gRPC equivalent of [`http::StatusCode`].
    ///
    /// Loosely based on https://github.com/grpc/grpc/blob/master/doc/http-grpc-status-mapping.md
    /// with the following changes:
    ///
    /// - http::StatusCode::NOT_FOUND => Code::NotFound (instead of Code::Unimplemented)
    /// - http::StatusCode::UNPROCESSABLE_ENTITY => Code::InvalidArgument (instead of Code::Unknown)
    /// - http::StatusCode::NOT_IMPLEMENTED => Code::Unimplemented (instead of Code::Unknown)
    /// - http::StatusCode::INTERNAL_SERVER_ERROR => Code::Internal (instead of Code::Unknown)
    /// - http::StatusCode::OK => Code::Ok (instead of Code::Unknown)
    pub fn from_http(http_code: http::StatusCode) -> Code {
        match http_code {
            http::StatusCode::OK => Code::Ok, // Code::Unknown
            http::StatusCode::BAD_REQUEST => Code::Internal,
            http::StatusCode::UNPROCESSABLE_ENTITY => Code::InvalidArgument, // Code::Unknown
            http::StatusCode::UNAUTHORIZED => Code::Unauthenticated,
            http::StatusCode::FORBIDDEN => Code::PermissionDenied,
            http::StatusCode::NOT_FOUND => Code::NotFound, // Code::Unimplemented,
            http::StatusCode::TOO_MANY_REQUESTS
            | http::StatusCode::BAD_GATEWAY
            | http::StatusCode::SERVICE_UNAVAILABLE
            | http::StatusCode::GATEWAY_TIMEOUT => Code::Unavailable,
            http::StatusCode::INTERNAL_SERVER_ERROR => Code::Internal, // Code::Unknown,
            http::StatusCode::NOT_IMPLEMENTED => Code::Unimplemented,  // Code::Unknown,
            _ => Code::Unknown,
        }
    }

    fn from_http_u16(code: u16) -> Result<Code, Error> {
        let status_code = http::StatusCode::from_u16(code)
            .map_err(|_| Error::Invalid("invalid status code".into()))?;
        Ok(Code::from_http(status_code))
    }

    /// Get description of this `Code`.
    pub fn description(&self) -> &'static str {
        match self {
            Code::Ok => "The operation completed successfully",
            Code::Cancelled => "The operation was cancelled",
            Code::Unknown => "Unknown error",
            Code::InvalidArgument => "Client specified an invalid argument",
            Code::DeadlineExceeded => "Deadline expired before operation could complete",
            Code::NotFound => "Some requested entity was not found",
            Code::AlreadyExists => "Some entity that we attempted to create already exists",
            Code::PermissionDenied => {
                "The caller does not have permission to execute the specified operation"
            }
            Code::ResourceExhausted => "Some resource has been exhausted",
            Code::FailedPrecondition => {
                "The system is not in a state required for the operation's execution"
            }
            Code::Aborted => "The operation was aborted",
            Code::OutOfRange => "Operation was attempted past the valid range",
            Code::Unimplemented => "Operation is not implemented or not supported",
            Code::Internal => "Internal error",
            Code::Unavailable => "The service is currently unavailable",
            Code::DataLoss => "Unrecoverable data loss or corruption",
            Code::Unauthenticated => "The request does not have valid authentication credentials",
        }
    }

    /// Returns [`http::HeaderValue`] representation.
    pub fn to_header_value(self) -> http::HeaderValue {
        match self {
            Code::Ok => http::HeaderValue::from_static("0"),
            Code::Cancelled => http::HeaderValue::from_static("1"),
            Code::Unknown => http::HeaderValue::from_static("2"),
            Code::InvalidArgument => http::HeaderValue::from_static("3"),
            Code::DeadlineExceeded => http::HeaderValue::from_static("4"),
            Code::NotFound => http::HeaderValue::from_static("5"),
            Code::AlreadyExists => http::HeaderValue::from_static("6"),
            Code::PermissionDenied => http::HeaderValue::from_static("7"),
            Code::ResourceExhausted => http::HeaderValue::from_static("8"),
            Code::FailedPrecondition => http::HeaderValue::from_static("9"),
            Code::Aborted => http::HeaderValue::from_static("10"),
            Code::OutOfRange => http::HeaderValue::from_static("11"),
            Code::Unimplemented => http::HeaderValue::from_static("12"),
            Code::Internal => http::HeaderValue::from_static("13"),
            Code::Unavailable => http::HeaderValue::from_static("14"),
            Code::DataLoss => http::HeaderValue::from_static("15"),
            Code::Unauthenticated => http::HeaderValue::from_static("16"),
        }
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.description(), f)
    }
}
