use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};

use bytes::{Bytes, BytesMut};
use futures::{future::BoxFuture, StreamExt};
use http::{HeaderMap, HeaderValue};
use http_body::Frame;
use http_body_util::{BodyExt, StreamBody};
use hyper::{body::Incoming, service::Service};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::body::BoxBody;
use tracing::debug;

use crate::{Headers, MockSet, Request};

#[derive(Debug, Clone)]
pub struct GrpcMockService {
    pub mocks: Arc<RwLock<MockSet>>,
}

impl GrpcMockService {
    pub fn new(mocks: Arc<RwLock<MockSet>>) -> Self {
        Self { mocks }
    }
}

impl Service<http::Request<Incoming>> for GrpcMockService {
    type Response = http::Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: http::Request<Incoming>) -> Self::Future {
        let mocks = self.mocks.clone();
        let fut = async move {
            debug!("handling request");
            let headers: Headers = req.headers().into();
            if !headers.has_content_type("application/grpc") {
                return Ok(invalid_content_type_response());
            }
            let (parts, body) = req.into_parts();
            let mut stream = body.into_data_stream();

            // Create response stream
            let (response_tx, response_rx) =
                mpsc::channel::<Result<Frame<Bytes>, tonic::Status>>(32);
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

                    // Match request to mock response
                    request = request.with_body(buf.clone().freeze());
                    let response = mocks.read().unwrap().match_to_response(&request);

                    if let Some(mut response) = response {
                        matched = true;
                        debug!("mock found, sending response");
                        // build headers
                        let mut headers = HeaderMap::from(response.headers().clone());
                        headers.insert("grpc-status", response.status().as_grpc_i32().into());
                        if let Some(message) = response.message() {
                            headers.insert("grpc-message", HeaderValue::from_str(message).unwrap());
                        }
                        // send data frames
                        if !response.body().is_empty() {
                            while let Some(chunk) = response.body.next().await {
                                let _ = response_tx.send(Ok(Frame::data(chunk))).await;
                            }
                        }
                        // send trailers frame
                        let _ = response_tx.send(Ok(Frame::trailers(headers))).await;
                        // Clear body buffer
                        buf.clear();
                    }
                }

                debug!("request stream closed");
                if !matched {
                    debug!("no mocks found, sending error");
                    dbg!(request);
                    dbg!(mocks);
                    let _ = response_tx
                        .send(Ok(Frame::trailers(mocks_not_found_headers())))
                        .await;
                }
            });

            Ok(response)
        };
        Box::pin(fut)
    }
}

fn invalid_content_type_response() -> http::Response<BoxBody> {
    http::Response::builder()
        .header("content-type", "application/grpc")
        .header("grpc-status", tonic::Code::InvalidArgument as i32)
        .header("grpc-message", "invalid content-type")
        .body(tonic::body::empty_body())
        .unwrap()
}

fn mocks_not_found_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("grpc-status", (tonic::Code::NotFound as i32).into());
    headers.insert("grpc-message", HeaderValue::from_static("mock not found"));
    headers
}
