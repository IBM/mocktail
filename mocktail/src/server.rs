use std::{
    net::SocketAddr,
    sync::{RwLock, RwLockWriteGuard},
};

use async_trait::async_trait;
use url::Url;

use crate::{mock::MockSet, Error};

mod grpc;
pub use grpc::GrpcMockServer;

mod http;
pub use http::HttpMockServer;

/// Server state.
#[derive(Debug)]
pub struct ServerState {
    pub mocks: RwLock<MockSet>,
}

impl ServerState {
    pub fn new(mocks: MockSet) -> Self {
        Self {
            mocks: RwLock::new(mocks),
        }
    }
}

#[async_trait]
pub trait MockServer {
    async fn start(&self) -> Result<(), Error>;

    /// Returns the server's service name.
    fn name(&self) -> &str;

    /// Returns the server's address.
    fn addr(&self) -> SocketAddr;

    /// Returns the url for a path.
    fn url(&self, path: &str) -> Url;

    fn mocks(&self) -> RwLockWriteGuard<'_, MockSet>;
}
