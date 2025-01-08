use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use http::{Request, Response};
use http_body_util::BodyExt;
use tonic::{
    body::BoxBody,
    codegen::{http, Body, BoxFuture, StdError},
    Code,
};
use tracing::debug;

use crate::{
    mock::{MockPath, MockSet},
    utils::{find_available_port, tonic::CodeExt},
    Error,
};

use super::ServerState;

/// A mock gRPC server.
#[derive(Clone)]
pub struct GrpcMockServer {
    name: &'static str,
    addr: SocketAddr,
    state: Arc<ServerState>,
}

impl GrpcMockServer {
    /// Creates a new [`GrpcMockServer`].
    pub fn new(name: &'static str, mocks: MockSet) -> Result<Self, Error> {
        let port = find_available_port().unwrap();
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        Ok(Self {
            name,
            addr,
            state: Arc::new(ServerState::new(mocks)),
        })
    }

    /// Returns the server's service name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the server's address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl GrpcMockServer {
    /// Handles a client request, returning a mock response.
    pub fn handle<B>(&self, req: Request<B>) -> BoxFuture<Response<BoxBody>, Infallible>
    where
        B: Body + Send + 'static,
        B::Data: Send,
        B::Error: Into<StdError> + Send + std::fmt::Debug + 'static,
    {
        let state = self.state.clone();
        let fut = async move {
            let path: MockPath = (req.method().clone(), req.uri().path().to_string()).into();
            debug!(?path, "handling grpc request");

            // Collect request body
            let body = req.into_body().collect().await.unwrap().to_bytes();

            // Match to mock and send response
            if let Some(mock) = state.mocks.find(&path, &body) {
                let mut builder = Response::builder()
                    .header("content-type", "application/grpc")
                    .header("grpc-status", Code::from_http(mock.response.code()) as i32);
                // TODO: insert headers from mock
                if let Some(error) = &mock.response.error {
                    builder = builder.header("grpc-message", error);
                }
                Ok(builder.body(mock.response.body().to_tonic_boxed()).unwrap())
            } else {
                // Request not matched to mock, send error response
                Ok(Response::builder()
                    .header("content-type", "application/grpc")
                    .header("grpc-status", Code::NotFound as i32)
                    .body(tonic::body::empty_body())
                    .unwrap())
            }
        };
        Box::pin(fut)
    }
}
