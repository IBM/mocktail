#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use mocktail::prelude::*;

    #[tokio::test]
    async fn test_simple_server_streaming() -> Result<(), Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::new(Method::POST, "/server-stream"),
            Mock::new(
                MockRequest::empty(),
                MockResponse::stream(["msg1", "msg2", "msg3"]),
            ),
        );
        let server = HttpMockServer::new("simple", mocks)?;
        server.start().await?;

        let client = reqwest::Client::new();
        let response = client.post(server.url("/server-stream")).send().await?;
        assert!(response.status() == StatusCode::OK);
        let mut stream = response.bytes_stream();
        while let Some(Ok(chunk)) = stream.next().await {
            println!("recv: {chunk:?}");
        }

        Ok(())
    }
}
