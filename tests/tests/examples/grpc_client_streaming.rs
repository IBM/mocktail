use mocktail::prelude::*;
use test_log::test;
use tests::pb::{hello_client::HelloClient, HelloClientStreamingResponse, HelloRequest};
use tonic::transport::Channel;

#[test(tokio::test)]
async fn test_client_streaming() -> Result<(), anyhow::Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/example.Hello/HelloClientStreaming").pb_stream([
            HelloRequest {
                name: "Mateus".into(),
            },
            HelloRequest {
                name: "Paulo".into(),
            },
        ]);
        then.pb(HelloClientStreamingResponse {
            messages: vec!["Hello Mateus!".into(), "Hello Paulo!".into()],
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
                name: "Mateus".into(),
            },
            HelloRequest {
                name: "Paulo".into(),
            },
        ]))
        .await;
    dbg!(&result);
    assert!(result.is_ok());

    Ok(())
}
