// use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HelloRequest {
//     pub name: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HelloResponse {
//     pub message: String,
// }

// #[cfg(test)]
// mod tests {
//     use mocktail::prelude::*;

//     use super::*;

//     #[test_log::test(tokio::test)]
//     async fn test_hello_simple() -> Result<(), Error> {
//         let mut mocks = MockSet::new();
//         mocks.insert(
//             MockPath::post("/hello"),
//             Mock::new(
//                 MockRequest::json(HelloRequest { name: "Dan".into() }),
//                 MockResponse::json(HelloResponse {
//                     message: "Hello Dan!".into(),
//                 }),
//             ),
//         );
//         let server = HttpMockServer::new("hello", mocks)?;
//         server.start().await?;

//         let client = reqwest::Client::new();

//         let response = client
//             .post(server.url("/hello"))
//             .json(&HelloRequest { name: "Dan".into() })
//             .send()
//             .await?;
//         assert!(response.status() == StatusCode::OK);
//         let body = response.json::<HelloResponse>().await?;
//         dbg!(&body);

//         let response = client
//             .post(server.url("/hello"))
//             .json(&HelloRequest {
//                 name: "Missing".into(),
//             })
//             .send()
//             .await?;
//         assert!(response.status() == StatusCode::NOT_FOUND);

//         // Clear the mocks on the server
//         server.mocks().clear();

//         assert!(server.mocks().is_empty());

//         // Add a new mock to the server
//         server.mocks().insert(
//             MockPath::post("/hello"),
//             Mock::new(
//                 MockRequest::json(HelloRequest {
//                     name: "There".into(),
//                 }),
//                 MockResponse::json(HelloResponse {
//                     message: "Hello There!".into(),
//                 }),
//             ),
//         );

//         let response = client
//             .post(server.url("/hello"))
//             .json(&HelloRequest {
//                 name: "There".into(),
//             })
//             .send()
//             .await?;
//         assert!(response.status().is_success());
//         let body = response.json::<HelloResponse>().await?;
//         dbg!(&body);

//         Ok(())
//     }
// }
