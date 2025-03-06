use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::{Arc, RwLockWriteGuard},
    time::Duration,
};

use async_trait::async_trait;
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Incoming, server::conn::http1, service::Service};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use tracing::{debug, error, info};
use url::Url;

use super::{MockServer, ServerState};
use crate::{
    mock::{HyperBoxBody, MockPath, MockSet},
    utils::find_available_port,
    Error,
};

/// A mock HTTP server.
pub struct HttpMockServer {
    name: &'static str,
    addr: SocketAddr,
    base_url: Url,
    state: Arc<ServerState>,
}

impl HttpMockServer {
    /// Creates a new [`HttpMockServer`].
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
impl MockServer for HttpMockServer {
    async fn start(&self) -> Result<(), Error> {
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
            let path = MockPath::from_request(&req);
            // let headers = req.headers().clone();
            debug!(?path, "handling http request");

            // Collect request body
            let body = req.into_body().collect().await.unwrap().to_bytes();

            // Match to mock and send response
            let mock = state
                .mocks
                .read()
                .unwrap()
                .match_by_body(&path, &body)
                .cloned();
            if let Some(mock) = mock {
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
