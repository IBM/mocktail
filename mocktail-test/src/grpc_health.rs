mod pb {
    tonic::include_proto!("grpc.health.v1");
}

#[cfg(test)]
mod tests {
    use mocktail::prelude::*;
    use tonic::transport::Channel;

    use super::pb::{health_client::HealthClient, HealthCheckRequest, HealthCheckResponse};

    #[test_log::test(tokio::test)]
    async fn test_health() -> Result<(), anyhow::Error> {
        // Create a mockset
        let mut mocks = MockSet::new();

        // Build a mock
        // let mock = Mock::new(|when, then| {
        //     when.method(Method::POST)
        //         .path("/grpc.health.v1.Health/Check")
        //         .pb(HealthCheckRequest { service: "".into() });
        //     then.pb(HealthCheckResponse { status: 1 });
        // });
        // Insert a mock
        // mocks.insert(mock);

        // Build and insert a mock using MockSet::mock()
        mocks.mock(|when, then| {
            when.post() // alt: when.method(Method::POST)
                .path("/grpc.health.v1.Health/Check")
                .pb(HealthCheckRequest { service: "".into() }); // alt: when.body(Body::pb(v))
            then.pb(HealthCheckResponse { status: 1 });
        });

        // Create server
        let server = MockServer::grpc("grpc.health.v1.Health").with_mocks(mocks);
        server.start().await?;

        // NOTE: mocks can also be built and insert directly to the server using server.mock()

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.port().unwrap()))?
            .connect()
            .await?;
        let mut client = HealthClient::new(channel);

        let result = client
            .check(HealthCheckRequest { service: "".into() })
            .await;
        dbg!(&result);
        assert!(result.is_ok());

        Ok(())
    }
}
