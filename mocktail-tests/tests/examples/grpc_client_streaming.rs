use anyhow::Error;
use mocktail::prelude::*;
use mocktail_tests::pb::{hello_client::HelloClient, HelloClientStreamingResponse, HelloRequest};
use test_log::test;
use tonic::transport::Channel;

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

    let server = MockServer::new("example.Hello").grpc().with_mocks(mocks);
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
