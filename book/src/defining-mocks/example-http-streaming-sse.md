# Example: HTTP Streaming (SSE)

This example demonstrates how to mock a Server-Sent Events (SSE) HTTP streaming endpoint using `bytes_stream`.

```rust
use eventsource_stream::{Event, Eventsource};
use futures::StreamExt;
use mocktail::prelude::*;

#[tokio::test]
async fn test_sse_stream() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a POST endpoint that streams Server-Sent Events (SSE)
    mocks.mock(|when, then| {
        when.post().path("/sse-stream");
        then.bytes_stream([
            "data: msg1\n\n",
            "data: msg2\n\n",
            "data: msg3\n\n",
            "event: error\ndata: internal error\n\n",
        ]);
    });

    // Create and start the HTTP mock server
    let server = MockServer::new_http("sse").with_mocks(mocks);
    server.start().await?;

    // Create an HTTP client
    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Send a request to the SSE endpoint
    let response = client.post(server.url("/sse-stream")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);

    // Read the incoming SSE events from the response stream
    let mut events: Vec<Event> = Vec::with_capacity(4);
    let mut stream = response.bytes_stream().eventsource();
    while let Some(Ok(msg)) = stream.next().await {
        events.push(msg);
    }

    assert_eq!(events.len(), 4);

    Ok(())
}
```

See [`http_streaming.rs`](https://github.com) in `mocktail-tests` for the full test suite.
