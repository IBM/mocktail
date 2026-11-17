# Example: HTTP Simple

This example demonstrates how to define and test unary HTTP mocks with JSON and plain text payloads, as well as custom error responses.

```rust
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

#[tokio::test]
async fn test_http_simple() -> Result<(), Box<dyn std::error::Error>> {
    // Create a mock set
    let mut mocks = MockSet::new();

    // Mock a POST request with a JSON body and response
    mocks.mock(|when, then| {
        when.post()
            .path("/hello")
            .json(HelloRequest { name: "dan".into() });
        then.json(HelloResponse {
            message: "hello dan!".into(),
        });
    });

    // Mock a GET request with a plain text response
    mocks.mock(|when, then| {
        when.get().path("/world");
        then.text("hello!");
    });

    // Mock an error response
    mocks.mock(|when, then| {
        when.get().path("/error");
        then.bad_request();
    });

    // Create and start the HTTP mock server
    let server = MockServer::new_http("hello").with_mocks(mocks);
    server.start().await?;

    // Create an HTTP client
    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Test the JSON POST mock
    let response = client
        .post(server.url("/hello"))
        .json(&HelloRequest { name: "dan".into() })
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.json::<HelloResponse>().await?;
    assert_eq!(
        res,
        HelloResponse {
            message: "hello dan!".into(),
        }
    );

    // Test the plain text GET mock
    let response = client.get(server.url("/world")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "hello!");

    // Test the error mock
    let response = client.get(server.url("/error")).send().await?;
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

    Ok(())
}
```

See [`http_unary.rs`](https://github.com) in `mocktail-tests` for the full test suite.
