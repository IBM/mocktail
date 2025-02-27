mod pb {
    tonic::include_proto!("example");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::{StreamExt, stream};
    use mocktail::prelude::*;
    use tokio_stream::wrappers::ReceiverStream;
    use tonic::transport::Channel;
    use tracing::debug;

    use super::pb::{HelloRequest, HelloResponse, hello_client::HelloClient};

    #[test_log::test(tokio::test)]
    async fn test_hello_unary() -> Result<(), anyhow::Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::new(Method::POST, "/example.Hello/HelloUnary"),
            Mock::new(
                MockRequest::pb(HelloRequest { name: "Dan".into() }),
                MockResponse::pb(HelloResponse {
                    message: "Hello Dan!".into(),
                }),
            ),
        );

        let server = GrpcMockServer::new("example.Hello", mocks)?;
        server.start().await?;

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.addr().port()))?
            .connect()
            .await?;
        let mut client = HelloClient::new(channel);

        let result = client
            .hello_unary(HelloRequest { name: "Dan".into() })
            .await;
        dbg!(&result);
        assert!(result.is_ok());

        let result = client
            .hello_unary(HelloRequest {
                name: "NotFound1".into(),
            })
            .await;
        dbg!(&result);
        assert!(result.is_err_and(|e| e.code() == tonic::Code::NotFound));

        Ok(())
    }

    #[test_log::test(tokio::test)]
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
        mocks.insert(
            MockPath::new(Method::POST, "/example.Hello/HelloBidiStreaming"),
            Mock::new(
                MockRequest::pb_stream([
                    HelloRequest {
                        name: "Mateus".into(),
                    },
                    HelloRequest {
                        name: "Paulo".into(),
                    },
                    HelloRequest {
                        name: "Shonda".into(),
                    },
                ]),
                MockResponse::pb_stream([
                    HelloResponse {
                        message: "Hello Mateus!".into(),
                    },
                    HelloResponse {
                        message: "Hello Paulo!".into(),
                    },
                    HelloResponse {
                        message: "Hello Shonda!".into(),
                    },
                ]),
            ),
        );
        let server = GrpcMockServer::new("example.Hello", mocks)?;
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

        // Client-streaming, should return error
        let result = client
            .hello_client_streaming(stream::iter(vec![
                HelloRequest {
                    name: "NotFound1".into(),
                },
                HelloRequest {
                    name: "NotFound2".into(),
                },
            ]))
            .await;
        dbg!(&result);
        assert!(result.is_err_and(|e| e.code() == tonic::Code::NotFound));

        // Server-streaming
        let response = client
            .hello_server_streaming(HelloRequest {
                name: "Dan, Gaurav".into(),
            })
            .await?;
        let mut stream = response.into_inner();
        let mut responses = Vec::with_capacity(2);
        while let Some(Ok(message)) = stream.next().await {
            debug!("[server-streaming] recv: {message:?}");
            responses.push(message);
        }
        assert!(responses.len() == 2);
        dbg!(&responses);

        // Bidi-streaming
        let (request_tx, request_rx) = tokio::sync::mpsc::channel(32);
        let request_stream = ReceiverStream::new(request_rx);
        let response = client.hello_bidi_streaming(request_stream).await?;
        let mut stream = response.into_inner();
        let mut responses = Vec::with_capacity(3);
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            for msg in [
                HelloRequest {
                    name: "Mateus".into(),
                },
                HelloRequest {
                    name: "Paulo".into(),
                },
                HelloRequest {
                    name: "Shonda".into(),
                },
            ] {
                let _ = request_tx.send(msg).await;
            }
        });

        while let Some(Ok(message)) = stream.next().await {
            debug!("[bidi-streaming] recv: {message:?}");
            responses.push(message);
        }
        assert!(responses.len() == 3);
        dbg!(&responses);

        Ok(())
    }
}
