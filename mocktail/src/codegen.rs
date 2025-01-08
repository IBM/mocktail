/// Generates a mock gRPC server.
#[macro_export]
macro_rules! generate_grpc_server {
    ($name:literal, $type:ident) => {
        #[derive(Clone)]
        pub struct $type($crate::server::GrpcMockServer);

        impl std::ops::Deref for $type {
            type Target = $crate::server::GrpcMockServer;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<B> tonic::codegen::Service<tonic::codegen::http::Request<B>> for $type
        where
            B: tonic::codegen::Body + Send + 'static,
            B::Data: Send,
            B::Error: Into<tonic::codegen::StdError> + Send + std::fmt::Debug + 'static,
        {
            type Response = tonic::codegen::http::Response<tonic::body::BoxBody>;
            type Error = std::convert::Infallible;
            type Future = tonic::codegen::BoxFuture<Self::Response, Self::Error>;

            fn poll_ready(
                &mut self,
                _cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<std::result::Result<(), Self::Error>> {
                std::task::Poll::Ready(Ok(()))
            }
            fn call(&mut self, req: tonic::codegen::http::Request<B>) -> Self::Future {
                self.0.handle(req)
            }
        }

        impl tonic::server::NamedService for $type {
            const NAME: &'static str = $name;
        }

        impl $type {
            pub fn new(mocks: MockSet) -> Result<Self, $crate::Error> {
                let server = $crate::server::GrpcMockServer::new($name, mocks)?;
                Ok(Self(server))
            }

            pub async fn start(&self) -> Result<(), $crate::Error> {
                let svc = self.clone();
                let addr = svc.addr();
                tokio::spawn(
                    tonic::transport::Server::builder()
                        .add_service(svc)
                        .serve(addr),
                );

                // Cushion for server to become ready, there is probably a better approach :)
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                Ok(())
            }
        }
    };
}
