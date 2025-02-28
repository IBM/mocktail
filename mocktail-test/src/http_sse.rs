#[cfg(test)]
mod tests {
    use eventsource_stream::Eventsource;
    use futures::StreamExt;
    use mocktail::prelude::*;
    use tracing::debug;

    #[test_log::test(tokio::test)]
    async fn test_sse_streaming() -> Result<(), Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::post("/sse-stream"),
            Mock::new(
                MockRequest::empty(),
                MockResponse::sse_stream([
                    Event::new("msg1"),
                    Event::new("msg2"),
                    Event::new("msg3"),
                    Event::new("internal error").with_event("error"),
                ]),
            ),
        );
        let server = HttpMockServer::new("sse", mocks)?;
        server.start().await?;

        let client = reqwest::Client::new();
        let response = client.post(server.url("/sse-stream")).send().await?;
        assert!(response.status() == StatusCode::OK);

        let mut events = Vec::with_capacity(4);
        let mut stream = response.bytes_stream().eventsource();
        while let Some(Ok(event)) = stream.next().await {
            debug!("[sse-streaming] recv: {event:?}");
            events.push(event);
        }
        dbg!(&events);
        assert!(events.len() == 4);

        Ok(())
    }
}
