# Example: gRPC Unary

This example demonstrates how to define and test unary gRPC mocks using protobuf payloads (`pb`) and custom gRPC error responses.

```rust
use mocktail::prelude::*;
use mocktail_tests::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};
use tonic::transport::Channel;

#[tokio::test]
async fn test_grpc_unary() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a unary gRPC method with a protobuf request and response
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary")
            .pb(HelloRequest { name: "dan".into() });
        then.pb(HelloResponse {
            message: "hello dan!".into(),
        });
    });

    // Mock a gRPC internal server error response
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary").pb(HelloRequest {
            name: "unexpected_error".into(),
        });
        then.internal_server_error().message("unexpected error");
    });

    // Create and start the gRPC mock server
    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    // Connect a Tonic gRPC client to the mock server
    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    // Test the successful unary gRPC mock
    let response = client
        .hello_unary(HelloRequest { name: "dan".into() })
        .await?
        .into_inner();
    assert_eq!(
        response,
        HelloResponse {
            message: "hello dan!".into(),
        }
    );

    // Test the error gRPC mock
    let result = client
        .hello_unary(HelloRequest {
            name: "unexpected_error".into(),
        })
        .await;
    assert!(result.is_err_and(|e| {
        e.code() == tonic::Code::Internal && e.message() == "unexpected error"
    }));

    Ok(())
}
```

See [`grpc_unary.rs`](https://github.com) in `mocktail-tests` for the full test suite.
