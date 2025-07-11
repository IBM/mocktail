use anyhow::Error;
use mocktail::prelude::*;
use mocktail_tests::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};
use test_log::test;
use tonic::transport::Channel;

#[test(tokio::test)]
async fn test_unary() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary")
            .pb(HelloRequest { name: "dan".into() });
        then.pb(HelloResponse {
            message: "hello dan!".into(),
        });
    });

    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    let result = client
        .hello_unary(HelloRequest { name: "dan".into() })
        .await;
    assert!(result.is_ok());
    let res = result.unwrap().into_inner();
    assert_eq!(
        res,
        HelloResponse {
            message: "hello dan!".into(),
        }
    );

    Ok(())
}

#[test(tokio::test)]
async fn test_unary_errors() -> Result<(), anyhow::Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloUnary").pb(HelloRequest {
            name: "unexpected_error".into(),
        });
        then.internal_server_error().message("unexpected error");
    });

    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    let result = client
        .hello_unary(HelloRequest {
            name: "unexpected_error".into(),
        })
        .await;
    assert!(result.is_err_and(|e| {
        e.code() == tonic::Code::Internal && e.message() == "unexpected error"
    }));

    // Mock not found
    let result = client
        .hello_unary(HelloRequest {
            name: "does_not_exist".into(),
        })
        .await;
    assert!(
        result.is_err_and(|e| e.code() == tonic::Code::NotFound && e.message() == "mock not found")
    );

    Ok(())
}
