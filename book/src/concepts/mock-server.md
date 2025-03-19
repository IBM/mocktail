# Mock Server

The mock server is a simple, lightweight HTTP server designed for serving mocks. It has 2 service implementations: `HttpMockService` and `GrpcMockService`. The server supports HTTP/1 and HTTP/2.

## gRPC
By default, the `HttpMockService` is used for serving regular HTTP mocks. For gRPC, set the `grpc()` option on the server to enable the `GrpcMockService`. You can use tonic to connect to the gRPC service.

```rust
    let server = MockServer::new().grpc();
    let url = format!("http://0.0.0.0:{}", server.port().unwrap());
    let channel = tonic::Channel::from_shared(url)?
        .connect()
        .await?;
    // Some client generated with tonic-build
    let mut client = ExampleClient::new(channel);
```

## TLS
TLS support is *not yet implemented*, but it will be added in the near future.