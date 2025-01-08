mod pb {
    tonic::include_proto!("example");
}

#[cfg(test)]
mod tests {
    use super::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};
    use futures::{stream, StreamExt};
    use mocktail::prelude::*;
    use tonic::transport::Channel;

    generate_grpc_server!("example.Hello", MockHelloServer);

    #[tokio::test]
    async fn test_hello_streaming() -> Result<(), anyhow::Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::new(Method::POST, "/example.Hello/HelloClientStreaming"),
            Mock::new(
                MockRequest::stream([
                    HelloRequest { name: "Dan".into() }.to_bytes(),
                    HelloRequest {
                        name: "Gaurav".into(),
                    }
                    .to_bytes(),
                ]),
                MockResponse::pb(HelloResponse {
                    message: "Hello Dan, Gaurav!".into(),
                }),
            ),
        );
        mocks.insert(
            MockPath::new(Method::POST, "/example.Hello/HelloServerStreaming"),
            Mock::new(
                MockRequest::pb(HelloRequest {
                    name: "Dan, Gaurav".into(),
                }),
                // This example uses MockRequest::pb_stream convenience method
                MockResponse::pb_stream([
                    HelloResponse {
                        message: "Hello Dan!".into(),
                    },
                    HelloResponse {
                        message: "Hello Gaurav!".into(),
                    },
                ]),
            ),
        );
        let server = MockHelloServer::new(mocks)?;
        server.start().await?;

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.addr().port()))?
            .connect()
            .await?;
        let mut client = HelloClient::new(channel);

        // Client-streaming
        let result = client
            .hello_client_streaming(stream::iter(vec![
                HelloRequest { name: "Dan".into() },
                HelloRequest {
                    name: "Gaurav".into(),
                },
            ]))
            .await;
        dbg!(&result);
        assert!(result.is_ok());

        // Server-streaming
        let response = client
            .hello_server_streaming(HelloRequest {
                name: "Dan, Gaurav".into(),
            })
            .await?;
        let mut stream = response.into_inner();
        while let Some(Ok(message)) = stream.next().await {
            println!("recv: {message:?}");
        }

        Ok(())
    }
}
