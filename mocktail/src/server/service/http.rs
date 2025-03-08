use std::sync::{Arc, RwLock};

use bytes::Bytes;
use futures::future::BoxFuture;
use http_body_util::{combinators::BoxBody, BodyExt, Empty};
use hyper::{body::Incoming, service::Service};
use tracing::debug;

use crate::{MockSet, Request};

//type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

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
    type Response = http::Response<BoxBody<Bytes, hyper::Error>>;
    type Error = hyper::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: http::Request<Incoming>) -> Self::Future {
        let (parts, body) = req.into_parts();

        let mocks = self.mocks.clone();
        let fut = async move {
            debug!("handling request");
            // Collect request body
            let body = body.collect().await.unwrap().to_bytes();
            // Match to mock and send response
            let request = Request::from_parts(parts).with_body(body);
            let response = mocks.read().unwrap().match_to_response(&request);
            if let Some(response) = response {
                let response = http::Response::builder()
                    .status(response.status.as_http())
                    .body(response.body().to_hyper_boxed())
                    .unwrap();
                // *response.headers_mut() = mock.response.headers().clone();
                // TODO: error message
                Ok(response)
            } else {
                // Request not matched to mock, send error response
                Ok(http::Response::builder()
                    .status(http::StatusCode::NOT_FOUND)
                    .body(empty_body())
                    .unwrap())
            }
        };
        Box::pin(fut)
    }
}

fn empty_body() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
