use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};

use bytes::{Bytes, BytesMut};
use futures::{StreamExt, future::BoxFuture};
use http::{HeaderMap, Request, Response};
use http_body::Frame;
use http_body_util::{BodyExt, StreamBody};
use hyper::{body::Incoming, server::conn::http2, service::Service};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::{net::TcpListener, sync::mpsc};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, info};
use url::Url;

use super::ServerState;
use crate::{
    Error,
    mock::{MockPath, MockSet, TonicBoxBody},
    utils::{find_available_port, has_content_type},
};

/// A mock gRPC server.
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

    pub async fn start(&self) -> Result<(), Error> {
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

        // Cushion for server to become ready, there is probably a better approach :)
        tokio::time::sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    /// Returns the server's service name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// Returns the server's address.
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    pub fn base_url(&self) -> Url {
        Url::parse(&format!("http://{}", self.addr())).unwrap()
    }

    pub fn url(&self, path: &str) -> Url {
        self.base_url().join(path).unwrap()
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
            let path: MockPath = (req.method().clone(), req.uri().path().to_string()).into();
            let headers = req.headers();
            debug!(?path, ?headers, "handling grpc request");

            if !has_content_type(headers, "application/grpc") {
                let response = Response::builder()
                    .header("content-type", "application/grpc")
                    .header("grpc-status", tonic::Code::InvalidArgument as i32)
                    .body(tonic::body::empty_body())
                    .unwrap();
                return Ok(response);
            }

            // Get request stream
            let mut stream = req.into_body().into_data_stream();

            // Create response channel
            let (response_tx, response_rx) =
                mpsc::channel::<Result<Frame<Bytes>, tonic::Status>>(32);

            // Create response wrapping response stream
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
                let mut sent = 0u32;
                while let Some(Ok(chunk)) = stream.next().await {
                    debug!(?chunk, "received chunk");
                    buf.extend(chunk);
                    // Attempt to match buffered data to mock
                    let body = buf.clone().freeze();
                    if let Some(mock) = state.mocks.find(&path, &body) {
                        // A matching mock has been found, send response data
                        debug!("mock found, sending response");
                        for data in mock.response.body().messages() {
                            let _ = response_tx.send(Ok(Frame::data(data))).await;
                            sent += 1;
                        }
                        buf.clear();
                    }
                }
                debug!("request stream closed");
                if sent == 0 {
                    debug!("no mocks found, sending error");
                    let mut trailers = HeaderMap::new();
                    trailers.insert("grpc-status", (tonic::Code::NotFound as i32).into());
                    let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                }
            });

            Ok(response)
        };
        Box::pin(fut)
    }
}
