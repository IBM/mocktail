use anyhow::Error;
use futures::StreamExt;
use mocktail::prelude::*;
use mocktail_tests::pb::{
    hello_client::HelloClient, HelloClientStreamingResponse, HelloRequest, HelloResponse,
    HelloServerStreamingRequest,
};
use test_log::test;
use tonic::transport::Channel;
use tracing::debug;

#[test(tokio::test)]
async fn test_client_streaming() -> Result<(), Error> {
    let mut mocks = MockSet::new();
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

    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    let result = client
        .hello_client_streaming(futures::stream::iter([
            HelloRequest {
                name: "mateus".into(),
            },
            HelloRequest {
                name: "paulo".into(),
            },
        ]))
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    Ok(())
}

#[test(tokio::test)]
async fn test_server_streaming() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloServerStreaming")
            .pb(HelloServerStreamingRequest {
                names: vec!["dan".into(), "gaurav".into()],
            });
        then.pb_stream([
            HelloResponse {
                message: "hello dan!".into(),
            },
            HelloResponse {
                message: "hello gaurav!".into(),
            },
        ]);
    });

    let server = MockServer::new_grpc("example.Hello").with_mocks(mocks);
    server.start().await?;

    let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
        .connect()
        .await?;
    let mut client = HelloClient::new(channel);

    let response = client
        .hello_server_streaming(HelloServerStreamingRequest {
            names: vec!["dan".into(), "gaurav".into()],
        })
        .await?;
    let mut stream = response.into_inner();

    let mut responses = Vec::with_capacity(2);
    while let Some(Ok(message)) = stream.next().await {
        debug!("recv: {message:?}");
        responses.push(message);
    }

    assert!(responses.len() == 2);
    dbg!(&responses);

    Ok(())
}
