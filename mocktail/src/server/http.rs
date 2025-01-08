use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};

use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Incoming, server::conn::http1, service::Service};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
use url::Url;

use crate::{
    mock::{HyperBoxBody, MockPath, MockSet},
    utils::find_available_port,
    Error,
};

use super::ServerState;

/// A mock HTTP server.
pub struct HttpMockServer {
    name: &'static str,
    addr: SocketAddr,
    state: Arc<ServerState>,
}

impl HttpMockServer {
    /// Creates a new [`HttpMockServer`].
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
        let service = HttpMockSvc {
            state: self.state.clone(),
        };
        let listener = TcpListener::bind(self.addr()).await?;
        info!("{} HTTP server listening on {}", self.name(), self.addr());

        // Spawn task to accept new connections
        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap(); // TODO: handle
                let io = TokioIo::new(stream);
                let service = service.clone();
                // Spawn task to serve connection
                tokio::spawn(async move {
                    // TODO: support both HTTP1 & HTTP2?
                    if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
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
struct HttpMockSvc {
    state: Arc<ServerState>,
}

impl Service<Request<Incoming>> for HttpMockSvc {
    type Response = Response<HyperBoxBody>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let state = self.state.clone();
        let fut = async move {
            let path: MockPath = (req.method().clone(), req.uri().path().to_string()).into();
            debug!(?path, "handling http request");

            // Collect request body
            let body = req.into_body().collect().await.unwrap().to_bytes();

            // Match to mock and send response
            if let Some(mock) = state.mocks.find(&path, &body) {
                let mut response = Response::builder()
                    .status(mock.response.code())
                    .body(mock.response.body().to_hyper_boxed())
                    .unwrap();
                *response.headers_mut() = mock.response.headers().clone();
                // TODO: error message
                Ok(response)
            } else {
                // Request not matched to mock, send error response
                Ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(empty_body())
                    .unwrap())
            }
        };
        Box::pin(fut)
    }
}

// Creates an empty body.
fn empty_body() -> HyperBoxBody {
    Empty::new().map_err(|err| match err {}).boxed()
}
