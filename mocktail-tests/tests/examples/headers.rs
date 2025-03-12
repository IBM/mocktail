use anyhow::Error;
use mocktail::prelude::*;
use test_log::test;

#[test(tokio::test)]
async fn test_headers() -> Result<(), Error> {
    let mut mocks = MockSet::new();

    // Mock with header matcher
    mocks.mock(|when, then| {
        when.post().path("/header").header("header1", "value1");
        then.text("you had the header!");
    });

    // Mock with headers matcher
    mocks.mock(|when, then| {
        when.post()
            .path("/headers")
            .headers([("header1", "value1"), ("header2", "value2")]);
        then.text("you had the headers!");
    });

    // Mock with header_exists matcher
    mocks.mock(|when, then| {
        when.post().path("/header-exists").header_exists("header1");
        then.text("you had the header!");
    });

    let server = MockServer::new("hello").with_mocks(mocks);
    server.start().await?;

    let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

    // Header
    let response = client
        .post(server.url("/header"))
        .header("header1", "value1")
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "you had the header!");
    dbg!(res);

    // Header, mock not found
    let response = client.post(server.url("/header")).send().await?;
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

    // Headers
    let response = client
        .post(server.url("/headers"))
        .headers(
            Headers::from_iter([
                ("header1", "value1"),
                ("header2", "value2"),
                ("header3", "value3"), // extra header, should be ok
            ])
            .into(),
        )
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "you had the headers!");
    dbg!(res);

    // Header exists
    let response = client
        .post(server.url("/header-exists"))
        .header("header1", "some_other_value")
        .send()
        .await?;
    assert_eq!(response.status(), http::StatusCode::OK);
    let res = response.text().await?;
    assert_eq!(res, "you had the header!");
    dbg!(res);

    Ok(())
}
