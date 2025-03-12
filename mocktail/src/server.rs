//! Mock server
use http_body::Body;
use hyper::{body::Incoming, service::Service};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::{conn, graceful::GracefulShutdown},
};
use rand::Rng;
use std::{
    cell::OnceCell,
    net::{SocketAddr, TcpStream},
    sync::{Arc, RwLock, RwLockWriteGuard},
    time::Duration,
};
use tokio::{net::TcpListener, sync::watch};
use tracing::{debug, error, info, warn};
use url::Url;

use crate::{
    mock::Mock,
    mock_builder::{Then, When},
    mock_set::MockSet,
    service::{GrpcMockService, HttpMockService},
    Error,
};

/// A mock server.
pub struct MockServer {
    name: &'static str,
    kind: ServerKind,
    addr: OnceCell<SocketAddr>,
    base_url: OnceCell<Url>,
    mocks: Arc<RwLock<MockSet>>,
    shutdown_tx: watch::Sender<()>,
}

impl MockServer {
    /// Creates a new [`MockServer`].
    pub fn new(name: &'static str) -> Self {
        let (shutdown_tx, _) = watch::channel(());
        Self {
            name,
            kind: ServerKind::Http,
            addr: OnceCell::new(),
            base_url: OnceCell::new(),
            mocks: Arc::new(RwLock::new(MockSet::default())),
            shutdown_tx,
        }
    }

    /// Sets the server type to gRPC.
    pub fn grpc(mut self) -> Self {
        self.kind = ServerKind::Grpc;
        self
    }

    /// Sets the server mocks.
    pub fn with_mocks(mut self, mocks: MockSet) -> Self {
        self.mocks = Arc::new(RwLock::new(mocks));
        self
    }

    pub async fn start(&self) -> Result<(), Error> {
        if self.addr().is_some() {
            return Err(Error::ServerError("already running".into()));
        }
        let port = find_available_port().unwrap();
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let base_url = Url::parse(&format!("http://{}", &addr)).unwrap();
        info!("starting {} [{}] server on {addr}", self.name(), &self.kind);
        let listener = TcpListener::bind(&addr).await?;
        let shutdown_rx = self.shutdown_tx.subscribe();
        match self.kind {
            ServerKind::Http => {
                let service = HttpMockService::new(self.mocks.clone());
                tokio::spawn(run_server(listener, self.kind, service, shutdown_rx));
            }
            ServerKind::Grpc => {
                let service = GrpcMockService::new(self.mocks.clone());
                tokio::spawn(run_server(listener, self.kind, service, shutdown_rx));
            }
        };
        // Wait for server to become ready
        for _ in 0..30 {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(10)).is_ok() {
                break;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        info!("{} server ready", self.name());

        self.addr.set(addr).unwrap();
        self.base_url.set(base_url).unwrap();

        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn addr(&self) -> Option<&SocketAddr> {
        self.addr.get()
    }

    pub fn port(&self) -> Option<u16> {
        self.addr.get().map(|v| v.port())
    }

    pub fn base_url(&self) -> Option<&Url> {
        self.base_url.get()
    }

    pub fn url(&self, path: &str) -> Url {
        if let Some(url) = self.base_url() {
            url.join(path).unwrap()
        } else {
            panic!("server not running");
        }
    }

    pub fn is_running(&self) -> bool {
        self.addr().is_some()
    }

    pub fn mocks(&self) -> RwLockWriteGuard<'_, MockSet> {
        self.mocks.write().unwrap()
    }

    /// Builds and inserts a mock.
    pub fn mock<F>(&mut self, f: F)
    where
        F: FnOnce(When, Then),
    {
        let mock = Mock::new(f);
        self.mocks().insert(mock);
    }

    /// Builds and inserts a mock with explicit priority.
    pub fn mock_with_priority<F>(&mut self, priority: u8, f: F)
    where
        F: FnOnce(When, Then),
    {
        let mock = Mock::new(f).with_priority(priority);
        self.mocks().insert(mock);
    }
}

#[derive(Debug, Clone, Copy)]
enum ServerKind {
    Http,
    Grpc,
}

impl std::fmt::Display for ServerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerKind::Http => write!(f, "http"),
            ServerKind::Grpc => write!(f, "grpc"),
        }
    }
}

/// Runs the main server loop to accept and serve connections.
async fn run_server<S, B>(
    listener: TcpListener,
    server_kind: ServerKind,
    service: S,
    mut shutdown_rx: watch::Receiver<()>,
) -> Result<(), Error>
where
    S: Service<http::Request<Incoming>, Response = http::Response<B>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    B: Body + Send + 'static,
    B::Data: Send + 'static,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let executor = TokioExecutor::new();
    let builder = match server_kind {
        ServerKind::Http => conn::auto::Builder::new(executor),
        ServerKind::Grpc => conn::auto::Builder::new(executor).http2_only(),
    };
    let graceful = GracefulShutdown::new();

    // Spawn task to accept new connections
    tokio::spawn(async move {
        loop {
            tokio::select! {
                connection = listener.accept() => {
                    let (stream, addr) = match connection {
                        Ok(conn) => conn,
                        Err(err) => {
                            error!("connection accept error: {err}");
                            continue;
                        }
                    };
                    debug!("connection accepted: {addr}");
                    let io = TokioIo::new(stream);
                    let service = service.clone();
                    let connection = graceful.watch(builder.serve_connection(io, service).into_owned());
                    // Spawn task to serve connection
                    tokio::spawn(async move {
                        if let Err(err) = connection.await {
                            error!("connection error: {err}");
                        }
                        debug!("connection dropped: {addr}");
                    });
                },
                _ = shutdown_rx.changed() => {
                    warn!("shutting down");
                    drop(listener);
                    break;
                }
            }
        }

        tokio::select! {
            _ = graceful.shutdown() => {
                debug!("connections gracefully closed");
            },
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                debug!("timed out waiting for connections to close");
            }
        }
    });

    Ok(())
}

fn find_available_port() -> Option<u16> {
    let mut rng = rand::rng();
    loop {
        let port: u16 = rng.random_range(40000..60000);
        if port_is_available(port) {
            return Some(port);
        }
    }
}

fn port_is_available(port: u16) -> bool {
    std::net::TcpListener::bind(("0.0.0.0", port)).is_ok()
}
