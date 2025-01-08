use crate::mock::MockSet;

mod grpc;
pub use grpc::GrpcMockServer;

mod http;
pub use http::HttpMockServer;

/// Server state.
#[derive(Debug)]
pub struct ServerState {
    pub mocks: MockSet,
}

impl ServerState {
    pub fn new(mocks: MockSet) -> Self {
        Self { mocks }
    }
}
