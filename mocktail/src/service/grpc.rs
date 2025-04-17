//! Mock gRPC service
use std::{convert::Infallible, sync::Arc};

use bytes::{Bytes, BytesMut};
use futures::{future::BoxFuture, StreamExt};
use http::{HeaderMap, HeaderValue};
use http_body::Frame;
use http_body_util::{BodyExt, StreamBody};
use hyper::{body::Incoming, service::Service};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::debug;

use crate::{request::Request, server::MockServerState, service::http::empty, Code};

use super::http::BoxBody;

/// Mock gRPC service.
#[derive(Debug, Clone)]
pub struct GrpcMockService {
    state: Arc<MockServerState>,
}

impl GrpcMockService {
    pub fn new(state: Arc<MockServerState>) -> Self {
        Self { state }
    }
}

impl Service<http::Request<Incoming>> for GrpcMockService {
    type Response = http::Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: http::Request<Incoming>) -> Self::Future {
        let state = self.state.clone();
        let fut = async move {
            debug!(?req, "handling request");

            if req.method() != http::Method::POST {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::METHOD_NOT_ALLOWED)
                    .header("Allow", "POST")
                    .body(empty())
                    .unwrap());
            }
            let content_type = req.headers().get("content-type");
            if !content_type.is_some_and(|v| {
                v.to_str()
                    .unwrap_or_default()
                    .starts_with("application/grpc")
            }) {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::UNSUPPORTED_MEDIA_TYPE)
                    .header("Accept-Post", "application/grpc")
                    .body(empty())
                    .unwrap());
            }

            let (parts, body) = req.into_parts();
            let mut stream = body.into_data_stream();

            // Create response stream
            let (response_tx, response_rx) =
                mpsc::channel::<Result<Frame<Bytes>, hyper::Error>>(32);
            let response_stream = ReceiverStream::new(response_rx);
            let response_body = BoxBody::new(StreamBody::new(response_stream));
            let response = http::Response::builder()
                .header("content-type", "application/grpc")
                .body(response_body)
                .unwrap();

            // Spawn task to handle request
            tokio::spawn(async move {
                let mut request = Request::from_parts(parts);
                let mut matched = false;
                let mut buf = BytesMut::new();

                while let Some(Ok(chunk)) = stream.next().await {
                    debug!(?chunk, "received chunk");
                    // Add chunk to body buffer
                    buf.extend(chunk);

                    // Match request to mock
                    request = request.with_body(buf.clone().freeze());
                    let mock = state.mocks().match_by_request(&request);
                    if let Some(mock) = mock {
                        matched = true;
                        debug!("mock found, sending response");
                        let mut response = mock.response;
                        // Send data frames
                        if !response.body().is_empty() {
                            while let Some(chunk) = response.body.next().await {
                                let _ = response_tx.send(Ok(Frame::data(chunk))).await;
                            }
                        }
                        // Send trailers frame
                        let mut trailers = HeaderMap::from(response.headers().clone());
                        trailers
                            .insert("grpc-status", response.status().as_grpc().to_header_value());
                        if let Some(message) = response.message() {
                            trailers
                                .insert("grpc-message", HeaderValue::from_str(message).unwrap());
                        }
                        let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                        // Clear body buffer
                        buf.clear();
                    }
                }
                debug!("request stream closed");
                if !matched {
                    debug!(?request, "no mocks found, sending error");
                    let _ = response_tx
                        .send(Ok(Frame::trailers(mock_not_found_trailer())))
                        .await;
                }
            });

            Ok(response)
        };
        Box::pin(fut)
    }
}

fn mock_not_found_trailer() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("grpc-status", Code::NotFound.to_header_value());
    headers.insert("grpc-message", HeaderValue::from_static("mock not found"));
    headers
}
