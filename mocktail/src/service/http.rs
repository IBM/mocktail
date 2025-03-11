use std::{
    convert::Infallible,
    sync::{Arc, RwLock},
};

use bytes::{Bytes, BytesMut};
use futures::{future::BoxFuture, StreamExt};
use http::HeaderMap;
use http_body::Frame;
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::{body::Incoming, service::Service};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::debug;

use crate::{Headers, MockSet, Request};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[derive(Debug, Clone)]
pub struct HttpMockService {
    pub mocks: Arc<RwLock<MockSet>>,
}

impl HttpMockService {
    pub fn new(mocks: Arc<RwLock<MockSet>>) -> Self {
        Self { mocks }
    }
}

impl Service<http::Request<Incoming>> for HttpMockService {
    type Response = http::Response<BoxBody>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: http::Request<Incoming>) -> Self::Future {
        let mocks = self.mocks.clone();
        let fut = async move {
            debug!("handling request");
            let headers: Headers = req.headers().into();
            let (parts, body) = req.into_parts();

            if headers.has_chunked_encoding() {
                // Process body as streaming
                let mut stream = body.into_data_stream();

                // Create response stream
                let (response_tx, response_rx) =
                    mpsc::channel::<Result<Frame<Bytes>, hyper::Error>>(32);
                let response_stream = ReceiverStream::new(response_rx);
                let response_body = BoxBody::new(StreamBody::new(response_stream));
                let response = http::Response::builder().body(response_body).unwrap();

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
                            // Send data frames
                            if !response.body().is_empty() {
                                while let Some(chunk) = response.body.next().await {
                                    let _ = response_tx.send(Ok(Frame::data(chunk))).await;
                                }
                            }
                            if response.is_error() {
                                let message = response
                                    .message()
                                    .map(|s| Bytes::copy_from_slice(s.as_bytes()))
                                    .unwrap_or_default();
                                let _ = response_tx.send(Ok(Frame::data(message))).await;
                            }
                            // Send trailers frame
                            let trailers = HeaderMap::from(response.headers().clone());
                            let _ = response_tx.send(Ok(Frame::trailers(trailers))).await;
                            // Clear body buffer
                            buf.clear();
                        }
                    }
                    debug!("request stream closed");
                    if !matched {
                        debug!(?request, "no mocks found, sending error");
                        // Send data frame with message
                        let _ = response_tx
                            .send(Ok(Frame::data("mock not found".into())))
                            .await;
                    }
                });
                Ok(response)
            } else {
                // Process body as unary
                // Collect body
                let bytes = body.collect().await.unwrap().to_bytes();
                let request = Request::from_parts(parts).with_body(bytes);
                let response = mocks.read().unwrap().match_to_response(&request);
                if let Some(response) = response {
                    debug!("mock found, sending response");
                    let mut body = response.body().clone().as_bytes();
                    if response.is_error() {
                        if let Some(message) = response.message() {
                            body = Bytes::copy_from_slice(message.as_bytes());
                        }
                    }
                    let status = response.status().as_http();
                    let mut res = http::Response::builder()
                        .status(status)
                        .body(full(body))
                        .unwrap();
                    let headers = res.headers_mut();
                    for (name, value) in response.headers {
                        headers.insert::<http::HeaderName>(name.into(), value.into());
                    }
                    Ok(res)
                } else {
                    debug!(?request, "no mocks found, sending error");
                    Ok(http::Response::builder()
                        .status(http::StatusCode::NOT_FOUND)
                        .body(full(Bytes::copy_from_slice("mock not found".as_bytes())))
                        .unwrap())
                }
            }
        };
        Box::pin(fut)
    }
}

fn full(data: Bytes) -> BoxBody {
    Full::new(data).map_err(|err| match err {}).boxed()
}
