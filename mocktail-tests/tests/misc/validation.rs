use anyhow::Error;
use mocktail::prelude::*;
use test_log::test;

#[test(tokio::test)]
async fn test_grpc_service() -> Result<(), Error> {
    let server = MockServer::new_grpc("test");
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Invalid method
    let response = client.get(server.url("/hello")).send().await?;
    assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
    assert!(response
        .headers()
        .get("allow")
        .is_some_and(|value| value == "POST"));

    // Invalid content-type
    let response = client.post(server.url("/hello")).send().await?;
    assert_eq!(response.status(), http::StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert!(response
        .headers()
        .get("accept-post")
        .is_some_and(|value| value == "application/grpc"));

    Ok(())
}

#[test(tokio::test)]
async fn test_http_service() -> Result<(), Error> {
    let server = MockServer::new_http("test");
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Invalid method
    let response = client.patch(server.url("/hello")).send().await?;
    assert_eq!(response.status(), http::StatusCode::METHOD_NOT_ALLOWED);
    assert!(response.headers().get("allow").is_some());

    Ok(())
}
