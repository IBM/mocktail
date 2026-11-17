# Example: gRPC Client Streaming

This example demonstrates how to define and test a client-streaming gRPC mock using `pb_stream` on the request matcher and `pb` on the response.

```rust
use mocktail::prelude::*;
use mocktail_tests::pb::{
    hello_client::HelloClient, HelloClientStreamingResponse, HelloRequest,
};
use tonic::transport::Channel;

#[tokio::test]
async fn test_grpc_client_streaming() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a client-streaming gRPC method
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloClientStreaming").pb_stream([
            HelloRequest {
                name: "mateus".into(),
            },
            HelloRequest {
                name: "paulo".into(),
            },
        ]);
        then.pb(HelloClientStreamingResponse {
            messages: vec!["hello mateus!".into(), "hello paulo!".into()],
        });
    });

    // Create and start the gRPC mock server
    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    // Connect a Tonic gRPC client to the mock server
    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    // Send a stream of requests to the server
    let response = client
        .hello_client_streaming(futures::stream::iter([
            HelloRequest {
                name: "mateus".into(),
            },
            HelloRequest {
                name: "paulo".into(),
            },
        ]))
        .await?
        .into_inner();

    assert_eq!(
        response,
        HelloClientStreamingResponse {
            messages: vec!["hello mateus!".into(), "hello paulo!".into()],
        }
    );

    Ok(())
}
```

See [`grpc_streaming.rs`](https://github.com) in `mocktail-tests` for the full test suite.
