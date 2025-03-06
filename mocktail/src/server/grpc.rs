use std::{
    convert::Infallible,
    net::SocketAddr,
    sync::{Arc, RwLockWriteGuard},
    time::Duration,
};

use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use futures::{future::BoxFuture, StreamExt};
use http::{HeaderMap, HeaderValue, Request, Response};
use http_body::Frame;
use http_body_util::{BodyExt, StreamBody};
use hyper::{body::Incoming, server::conn::http2, service::Service};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, info};
use url::Url;

use super::{MockServer, ServerState};
use crate::{
    mock::{MockPath, MockSet, TonicBoxBody},
    utils::{find_available_port, has_content_type, tonic::CodeExt},
    Error,
};

/// A mock gRPC server.
pub struct GrpcMockServer {
    name: &'static str,
    addr: SocketAddr,
    base_url: Url,
    state: Arc<ServerState>,
}

impl GrpcMockServer {
    /// Creates a new [`GrpcMockServer`].
    pub fn new(name: &'static str, mocks: MockSet) -> Result<Self, Error> {
        let port = find_available_port().unwrap();
        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse().unwrap();
        let base_url = Url::parse(&format!("http://{}", &addr)).unwrap();
        let state = Arc::new(ServerState::new(mocks));
        Ok(Self {
            name,
            addr,
            base_url,
            state,
        })
    }
}

#[async_trait]
impl MockServer for GrpcMockServer {
    async fn start(&self) -> Result<(), Error> {
        let service = GrpcMockSvc {
            state: self.state.clone(),
        };
        let listener = TcpListener::bind(self.addr()).await?;
        info!("{} gRPC server listening on {}", self.name(), self.addr());

        // Spawn task to accept new connections
        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let io = TokioIo::new(stream);
                let service = service.clone();
                // Spawn task to serve connection
                tokio::spawn(async move {
                    if let Err(err) = http2::Builder::new(TokioExecutor::new())
                        .serve_connection(io, service)
                        .await
                    {
                        error!("Error serving connection: {:?}", err);
                    }
                });
            }
        });

        // Give the server time to become ready
        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(())
    }

    fn name(&self) -> &str {
        self.name
    }

    fn addr(&self) -> SocketAddr {
        self.addr
    }

    fn url(&self, path: &str) -> Url {
        self.base_url.join(path).unwrap()
    }

    fn mocks(&self) -> RwLockWriteGuard<'_, MockSet> {
        self.state.mocks.write().unwrap()
    }
}

#[derive(Debug, Clone)]
struct GrpcMockSvc {
    state: Arc<ServerState>,
}

impl Service<Request<Incoming>> for GrpcMockSvc {
    type Response = Response<tonic::body::BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let state = self.state.clone();
        let fut = async move {
            let path = MockPath::from_request(&req);
            // let headers = req.headers().clone();
            debug!(?path, "handling grpc request");

            if !has_content_type(req.headers(), "application/grpc") {
                let response = Response::builder()
                    .header("content-type", "application/grpc")
                    .header("grpc-status", tonic::Code::InvalidArgument as i32)
                    .header("grpc-message", "invalid content-type")
                    .body(tonic::body::empty_body())
                    .unwrap();
                return Ok(response);
            }

            // Get request stream
            let mut stream = req.into_body().into_data_stream();

            // Create response channel
            let (response_tx, response_rx) =
                mpsc::channel::<Result<Frame<Bytes>, tonic::Status>>(32);

            // Create response
            let response_body =
                TonicBoxBody::new(StreamBody::new(ReceiverStream::new(response_rx)));
            let response = Response::builder()
                .header("content-type", "application/grpc")
                .body(response_body)
                .unwrap();

            // Spawn task to handle request
            tokio::spawn(async move {
                // Consume request stream
                let mut buf = BytesMut::new();
                let mut matched = false;
                while let Some(Ok(chunk)) = stream.next().await {
                    debug!(?chunk, "received chunk");
                    buf.extend(chunk);
                    // Attempt to match buffered data to mock
                    let body = buf.clone().freeze();
                    let mock = state
                        .mocks
                        .read()
                        .unwrap()
                        .match_by_body(&path, &body)
                        .cloned();
                    if let Some(mock) = mock {
                        matched = true;
                        // A matching mock has been found, send response
                        debug!("mock found, sending response");
                        if mock.response.is_error() {
                            let grpc_code = tonic::Code::from_http(mock.response.code());
                            let mut trailers = HeaderMap::new();
                            trailers.insert("grpc-status", (grpc_code as i32).into());
                            if let Some(message) = mock.response.message() {
                                trailers.insert(
                                    "grpc-message",
                                    HeaderValue::from_str(message).unwrap(),
                                );
                            }
                            let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                        } else {
                            for chunk in mock.response.body().chunks() {
                                let _ = response_tx.send(Ok(Frame::data(chunk))).await;
                            }
                        }
                        buf.clear();
                    }
                }
                debug!("request stream closed");
                if !matched {
                    debug!("no mocks found, sending error");
                    let mut trailers = HeaderMap::new();
                    trailers.insert("grpc-status", (tonic::Code::NotFound as i32).into());
                    trailers.insert("grpc-message", HeaderValue::from_static("mock not found"));
                    let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                }
            });

            Ok(response)
        };
        Box::pin(fut)
    }
}
