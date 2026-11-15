# Example: HTTP Streaming (ndjson)

This example demonstrates how to define and test HTTP streaming mocks using newline-delimited JSON (`ndjson` / JSON lines).

```rust
use futures::{stream, Stream, StreamExt};
use mocktail::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloResponse {
    pub message: String,
}

fn json_lines_stream(
    messages: impl IntoIterator<Item = impl Serialize>,
) -> impl Stream<Item = Result<Vec<u8>, std::io::Error>> {
    let chunks = messages
        .into_iter()
        .map(|msg| {
            let mut bytes = serde_json::to_vec(&msg).unwrap();
            bytes.push(b'\n');
            Ok(bytes)
        })
        .collect::<Vec<Result<_, std::io::Error>>>();
    stream::iter(chunks)
}

#[tokio::test]
async fn test_json_lines_stream() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a POST endpoint that matches a JSON lines request stream
    // and responds with a JSON lines stream
    mocks.mock(|when, then| {
        when.post().path("/hello").json_lines_stream([
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

    // Create and start the HTTP mock server
    let server = MockServer::new_http("hello").with_mocks(mocks);
    server.start().await?;

    // Create an HTTP client
    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Build the input JSON lines stream and send the request
    let input_stream = json_lines_stream([
        HelloRequest { name: "dan".into() },
        HelloRequest {
            name: "mateus".into(),
        },
    ]);
    let response = client
        .post(server.url("/hello"))
        .body(reqwest::Body::wrap_stream(input_stream))
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::OK);

    // Collect the streamed response chunks
    let mut responses = Vec::with_capacity(2);
    let mut stream = response.bytes_stream();
    while let Some(Ok(msg)) = stream.next().await {
        responses.push(msg);
    }

    assert_eq!(responses.len(), 2);

    Ok(())
}
```

See [`http_streaming.rs`](https://github.com) in `mocktail-tests` for the full test suite.
