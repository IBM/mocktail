use anyhow::Error;
use mocktail::prelude::*;
use serde::{Deserialize, Serialize};
use test_log::test;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HelloResponse {
    pub message: String,
}

#[test(tokio::test)]
async fn test_unary() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/hello")
            .json(HelloRequest { name: "dan".into() });
        then.json(HelloResponse {
            message: "hello dan!".into(),
        });
    });

    let server = MockServer::new("hello").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

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

    Ok(())
}

#[test(tokio::test)]
async fn test_unary_errors() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.path("/hello").json(HelloRequest {
            name: "unexpected_error".into(),
        });
        then.internal_server_error().message("unexpected error");
    });

    let server = MockServer::new("hello").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    let response = client
        .post(server.url("/hello"))
        .json(&HelloRequest {
            name: "unexpected_error".into(),
        })
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
    let message = response.text().await?;
    assert_eq!(message, "unexpected error");

    // Mock not found
    let response = client
        .post(server.url("/hello"))
        .json(&HelloRequest {
            name: "does_not_exist".into(),
        })
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
    let message = response.text().await?;
    assert_eq!(message, "mock not found");

    Ok(())
}
