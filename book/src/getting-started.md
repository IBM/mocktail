# Getting Started

1. Add `mocktail` to `Cargo.toml` as a development dependency:
    ```toml
    [dev-dependencies]
    mocktail = { git = "https://github.com/IBM/mocktail.git", version = "0.2.4-alpha" }
    ```

2. Basic usage example:
    ```rust
    use mocktail::prelude::*;

    #[tokio::test]
    async fn test_example() -> Result<(), Box<dyn std::error::Error>> {
        // Create a mock set
        let mut mocks = MockSet::new();

        // Build a mock that returns a "hello world!" response
        // to POST requests to the /hello endpoint with the text "world"
        // in the body.
        mocks.mock(|when, then| {
            when.post().path("/hello").text("world");
            then.text("hello world!");
        });

        // Create and start a mock server
        let mut server = MockServer::new("example").with_mocks(mocks);
        server.start().await?;

        // Create a client
        let client = reqwest::Client::builder().http2_prior_knowledge().build()?;

        // Send a request that matches the mock created above
        let response = client
            .post(server.url("/hello"))
            .body("world")
            .send()
            .await?;
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = response.text().await?;
        assert_eq!(body, "hello world!");

        // Send a request that doesn't match a mock
        let response = client.get(server.url("/nope")).send().await?;
        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);

        // Mocks can also be registered to the server directly
        // Register a mock that will match the request above that returned 404
        server.mock(|when, then| {
            when.get().path("/nope");
            then.text("yep!");
        });

        // Send the request again, it should now match
        let response = client.get(server.url("/nope")).send().await?;
        assert_eq!(response.status(), http::StatusCode::OK);
        let body = response.text().await?;
        assert_eq!(body, "yep!");

        // Mocks can be cleared from the server, enabling server reuse
        server.mocks.clear();

        Ok(())
    }
    ```

3. For more examples, see [examples](https://github.com/IBM/mocktail/tree/main/mocktail-tests/tests/examples) in the `mocktail-tests` crate.