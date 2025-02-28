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
        let mut mocks = MockSet::new();
        mocks.insert(
            MockPath::post("/grpc.health.v1.Health/Check"),
            Mock::new(
                MockRequest::pb(HealthCheckRequest { service: "".into() }),
                MockResponse::pb(HealthCheckResponse { status: 1 }),
            ),
        );
        let server = GrpcMockServer::new("grpc.health.v1.Health", mocks)?;
        server.start().await?;

        // Create client
        let channel = Channel::from_shared(format!("http://0.0.0.0:{}", server.addr().port()))?
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
