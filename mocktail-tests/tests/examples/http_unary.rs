use anyhow::Error;
use http::HeaderMap;
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
        when.post()
            .path("/hello")
            .json(HelloRequest { name: "dan".into() });
        then.json(HelloResponse {
            message: "hello dan!".into(),
        });
    });
    mocks.mock(|when, then| {
        when.get().path("/world");
        then.text("hello!");
    });

    let server = MockServer::new_http("hello").with_mocks(mocks);
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

    let response = client.get(server.url("/world")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "hello!");

    Ok(())
}

#[test(tokio::test)]
async fn test_unary_errors() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.post().path("/hello").json(HelloRequest {
            name: "unexpected_error".into(),
        });
        then.internal_server_error().message("unexpected error");
    });
    mocks.mock(|when, then| {
        when.get().path("/error");
        then.bad_request();
    });

    let server = MockServer::new_http("hello").with_mocks(mocks);
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

    let response = client.get(server.url("/error")).send().await?;
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);

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

    // Clear server mocks
    server.mocks().clear();

    // Add a mock that responds with a 503 error on any
    // endpoint if the body is "give me an error"
    server.mocks().mock(|when, then| {
        when.text("give me an error");
        then.status(StatusCode::from_u16(503).unwrap());
    });
    let response = client
        .get(server.url("/path"))
        .body("give me an error")
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::SERVICE_UNAVAILABLE);

    Ok(())
}

#[test(tokio::test)]
async fn test_any() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.any();
        then.text("yo!");
    });

    let server = MockServer::new_http("any").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    let response = client.post(server.url("/asdf")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "yo!");

    let response = client.post(server.url("/whatever")).send().await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "yo!");

    Ok(())
}

#[test(tokio::test)]
async fn test_unary_headers() -> Result<(), Error> {
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.get().headers([
            ("content-type", "application/json"),
            ("Accept", "application/json, application/binary"),
        ]);
        then.text("yo!");
    });

    let server = MockServer::new_http("any").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());
    headers.insert(
        "accept",
        "application/json, application/binary".parse().unwrap(),
    );
    let response = client
        .get(server.url("/asdf"))
        .headers(headers)
        .send()
        .await?;

    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "yo!");

    let response = client.post(server.url("/whatever")).send().await?;
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    Ok(())
}
