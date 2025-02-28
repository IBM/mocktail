mod pb {
    tonic::include_proto!("example");
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use futures::{stream, StreamExt};
    use mocktail::prelude::*;
    use tokio_stream::wrappers::ReceiverStream;
    use tonic::transport::Channel;
    use tracing::debug;

    use super::pb::{hello_client::HelloClient, HelloRequest, HelloResponse};

    #[test_log::test(tokio::test)]
    async fn test_hello_unary() -> Result<(), anyhow::Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::post("/example.Hello/HelloUnary"),
            Mock::new(
                MockRequest::pb(HelloRequest { name: "Dan".into() }),
                MockResponse::pb(HelloResponse {
                    message: "Hello Dan!".into(),
                }),
            ),
        );
        mocks.insert(
            MockPath::post("/example.Hello/HelloUnary"),
            Mock::new(
                MockRequest::pb(HelloRequest {
                    name: "InternalError".into(),
                }),
                MockResponse::empty()
                    .with_code(StatusCode::INTERNAL_SERVER_ERROR)
                    .with_message("woops"),
            ),
        );
        // mocks.insert(
        //     MockPath::post("/example.Hello/HelloUnary"),
        //     Mock::new(
        //         MockRequest::pb(HelloRequest {
        //             name: "Header".into(),
        //         })
        //         .with_headers(HeaderMap::from_iter([(
        //             HeaderName::from_static("some-header"),
        //             HeaderValue::from_static(":D"),
        //         )])),
        //         MockResponse::pb(HelloResponse {
        //             message: "Hello Header!".into(),
        //         }),
        //     ),
        // );

        let server = GrpcMockServer::new("example.Hello", mocks)?;
        server.start().await?;

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.addr().port()))?
            .connect()
            .await?;
        let mut client = HelloClient::new(channel);

        // Success response
        let result = client
            .hello_unary(HelloRequest {
                name: "Header".into(),
            })
            .await;
        dbg!(&result);
        //assert!(result.is_ok());

        // Success response w/ header matching
        // let mut request = tonic::Request::new(HelloRequest { name: "Dan".into() });
        // request
        //     .metadata_mut()
        //     .insert("some-header", ":D".parse().unwrap());
        // let result = client.hello_unary(request).await;
        // dbg!(&result);
        // assert!(result.is_ok());

        // Error response (mock not found)
        let result = client
            .hello_unary(HelloRequest {
                name: "NotFoundError".into(),
            })
            .await;
        dbg!(&result);
        assert!(result.is_err_and(|e| e.code() == tonic::Code::NotFound));

        // Error response (internal)
        let result = client
            .hello_unary(HelloRequest {
                name: "InternalError".into(),
            })
            .await;
        dbg!(&result);
        assert!(
            result.is_err_and(|e| { e.code() == tonic::Code::Internal && e.message() == "woops" })
        );

        Ok(())
    }

    #[test_log::test(tokio::test)]
    async fn test_hello_streaming() -> Result<(), anyhow::Error> {
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::post("/example.Hello/HelloClientStreaming"),
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
            MockPath::post("/example.Hello/HelloServerStreaming"),
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
            MockPath::post("/example.Hello/HelloBidiStreaming"),
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
