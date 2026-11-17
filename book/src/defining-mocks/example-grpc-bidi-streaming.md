# Example: gRPC Bidi Streaming

This example demonstrates how to define and test a bidirectional (bidi) streaming gRPC mock using `pb_stream` on both the request matcher and the response.

```rust
use futures::StreamExt;
use mocktail::prelude::*;
use mocktail_tests::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};
use tonic::transport::Channel;

#[tokio::test]
async fn test_grpc_bidi_streaming() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a bidirectional streaming gRPC method
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloBidiStreaming").pb_stream([
            HelloRequest { name: "dan".into() },
            HelloRequest {
                name: "mateus".into(),
            },
        ]);
        then.pb_stream([
            HelloResponse {
                message: "hello dan!".into(),
            },
            HelloResponse {
                message: "hello mateus!".into(),
            },
        ]);
    });

    // Create and start the gRPC mock server
    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    // Connect a Tonic gRPC client to the mock server
    let channel = Channel::from_shared(format!("http://0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    // Send a stream of requests and read the response stream
    let response = client
        .hello_bidi_streaming(futures::stream::iter([
            HelloRequest { name: "dan".into() },
            HelloRequest {
                name: "mateus".into(),
            },
        ]))
        .await?;
    let mut stream = response.into_inner();

    let mut responses = Vec::with_capacity(2);
    while let Some(Ok(message)) = stream.next().await {
        responses.push(message);
    }

    assert_eq!(responses.len(), 2);

    Ok(())
}
```

See [`grpc_streaming.rs`](https://github.com) in `mocktail-tests` for the full test suite.
