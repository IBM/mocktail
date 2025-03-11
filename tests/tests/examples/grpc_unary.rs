use mocktail::prelude::*;
use test_log::test;
use tests::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};
use tonic::transport::Channel;

#[test(tokio::test)]
async fn test_unary() -> Result<(), anyhow::Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary")
            .pb(HelloRequest { name: "Dan".into() });
        then.pb(HelloResponse {
            message: "Hello Dan!".into(),
        });
    });

    let server = MockServer::new("example.Hello").grpc().with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    let result = client
        .hello_unary(HelloRequest { name: "Dan".into() })
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    Ok(())
}

#[test(tokio::test)]
async fn test_unary_errors() -> Result<(), anyhow::Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary").pb(HelloRequest {
            name: "InternalServerError".into(),
        });
        then.internal_server_error().message("ugh");
    });

    let server = MockServer::new("example.Hello").grpc().with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    // Mocked error response
    let result = client
        .hello_unary(HelloRequest {
            name: "InternalServerError".into(),
        })
        .await;
    dbg!(&result);
    assert!(result.is_err_and(|e| { e.code() == tonic::Code::Internal }));

    // Mock not found
    let result = client
        .hello_unary(HelloRequest {
            name: "IDontExist".into(),
        })
        .await;
    dbg!(&result);
    assert!(result.is_err_and(|e| e.code() == tonic::Code::NotFound));

    Ok(())
}
