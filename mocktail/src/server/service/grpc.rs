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

use crate::{utils::has_content_type, MockSet, Request};

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
            if !has_content_type(req.headers(), "application/grpc") {
                let response = http::Response::builder()
                    .header("content-type", "application/grpc")
                    .header("grpc-status", tonic::Code::InvalidArgument as i32)
                    .header("grpc-message", "invalid content-type")
                    .body(tonic::body::empty_body())
                    .unwrap();
                return Ok(response);
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
                    buf.extend(chunk);
                    request = request.with_body(buf.clone().freeze());
                    // Attempt to match request with buffered body data to mock
                    let response = mocks.read().unwrap().match_to_response(&request);
                    if let Some(response) = response {
                        matched = true;
                        debug!("mock found, sending response");
                        if response.is_ok() {
                            for chunk in response.body().chunks() {
                                let _ = response_tx.send(Ok(Frame::data(chunk))).await;
                            }
                        } else {
                            let mut trailers = HeaderMap::new();
                            trailers
                                .insert("grpc-status", (response.status.as_grpc() as i32).into());
                            if let Some(message) = response.message_header_value() {
                                trailers.insert("grpc-message", message);
                            }
                            let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                        }
                        buf.clear();
                    }
                }
                debug!("request stream closed");
                if !matched {
                    debug!("no mocks found, sending error");
                    let mut trailers = HeaderMap::new();
                    trailers.insert("grpc-status", (tonic::Code::NotFound as i32).into());
                    trailers.insert("grpc-message", HeaderValue::from_static("mock not found"));
                    let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                }
            });

            Ok(response)
        };
        Box::pin(fut)
    }
}

