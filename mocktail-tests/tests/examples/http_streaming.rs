use anyhow::Error;
use eventsource_stream::{Event, Eventsource};
use futures::{stream, StreamExt};
use mocktail::prelude::*;
use serde::{Deserialize, Serialize};
use test_log::test;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloResponse {
    pub message: String,
}

#[test(tokio::test)]
async fn test_json_lines_stream() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/hello").json_lines_stream([
            HelloRequest { name: "dan".into() },
            HelloRequest {
                name: "mateus".into(),
            },
        ]);
        then.json_lines_stream([
            HelloResponse {
                message: "hello dan!".into(),
            },
            HelloResponse {
                message: "hello mateus!".into(),
            },
        ]);
    });

    let server = MockServer::new("hello").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::new();

    let chunks = [
        HelloRequest { name: "dan".into() },
        HelloRequest {
            name: "mateus".into(),
        },
    ]
    .into_iter()
    .map(|msg| {
        let mut bytes = serde_json::to_vec(&msg).unwrap();
        bytes.push(b'\n');
        Ok(bytes)
    })
    .collect::<Vec<Result<_, std::io::Error>>>();

    let response = client
        .post(server.url("/hello"))
        .body(reqwest::Body::wrap_stream(stream::iter(chunks)))
        .send()
        .await?;
    dbg!(&response);
    assert_eq!(response.status(), http::StatusCode::OK);

    let mut stream = response.bytes_stream();
    let mut responses = Vec::with_capacity(2);
    while let Some(Ok(msg)) = stream.next().await {
        debug!("recv: {msg:?}");
        responses.push(msg);
    }

    assert!(responses.len() == 2);
    dbg!(&responses);

    Ok(())
}

#[test(tokio::test)]
async fn test_bytes_stream() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/hello").bytes_stream(["dan", "mateus"]);
        then.bytes_stream(["hello dan!", "hello mateus!"]);
    });

    let server = MockServer::new("hello").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::new();

    let chunks = ["dan", "mateus"]
        .into_iter()
        .map(Ok)
        .collect::<Vec<Result<_, std::io::Error>>>();

    let response = client
        .post(server.url("/hello"))
        .body(reqwest::Body::wrap_stream(stream::iter(chunks)))
        .send()
        .await?;
    dbg!(&response);
    assert_eq!(response.status(), http::StatusCode::OK);

    let mut stream = response.bytes_stream();
    let mut responses = Vec::with_capacity(2);
    while let Some(Ok(msg)) = stream.next().await {
        debug!("recv: {msg:?}");
        responses.push(msg);
    }

    assert!(responses.len() == 2);
    dbg!(&responses);

    Ok(())
}

#[test(tokio::test)]
async fn test_sse_stream() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/sse-stream");
        then.bytes_stream([
            "data: msg1\n\n",
            "data: msg2\n\n",
            "data: msg3\n\n",
            "event: error\ndata: internal error\n\n",
        ]);
    });

    let server = MockServer::new("sse").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::new();

    let response = client.post(server.url("/sse-stream")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);

    let mut events: Vec<Event> = Vec::with_capacity(4);
    let mut stream = response.bytes_stream().eventsource();
    while let Some(Ok(msg)) = stream.next().await {
        debug!("recv: {msg:?}");
        events.push(msg);
    }

    assert!(events.len() == 4);
    dbg!(&events);

    Ok(())
}
