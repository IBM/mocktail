#[cfg(test)]
mod tests {
    use eventsource_stream::{Event, Eventsource};
    use futures::StreamExt;
    use mocktail::prelude::*;

    #[tokio::test]
    async fn test_sse_server_streaming() -> Result<(), Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::new(Method::POST, "/sse-stream"),
            Mock::new(
                MockRequest::empty(),
                MockResponse::stream([
                    "data: msg1\n\n",
                    "data: msg2\n\n",
                    "data: msg3\n\n",
                    "event: error\ndata: internal error\n\n",
                ]),
            ),
        );
        let server = HttpMockServer::new("sse", mocks)?;
        server.start().await?;

        let client = reqwest::Client::new();
        let response = client.post(server.url("/sse-stream")).send().await?;
        assert!(response.status() == StatusCode::OK);

        let mut events: Vec<Event> = Vec::with_capacity(4);
        let mut stream = response.bytes_stream().eventsource();
        while let Some(Ok(event)) = stream.next().await {
            println!("recv event[{}]: {}", event.event, event.data);
            events.push(event);
        }
        dbg!(&events);
        assert_eq!(
            events,
            vec![
                Event {
                    event: "message".into(),
                    data: "msg1".into(),
                    ..Default::default()
                },
                Event {
                    event: "message".into(),
                    data: "msg2".into(),
                    ..Default::default()
                },
                Event {
                    event: "message".into(),
                    data: "msg3".into(),
                    ..Default::default()
                },
                Event {
                    event: "error".into(),
                    data: "internal error".into(),
                    ..Default::default()
                }
            ],
        );

        Ok(())
    }
}
