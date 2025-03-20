# Defining Mocks

Mocks are defined in Rust using a simple, ergonomic [builder-like API](./concepts/mock-builder.md).

You can define your mocks first, then create a mock server with your mock set:
```rust
    // Create a mock set
    let mut mocks = MockSet::new();
    // Build and insert a mock
    mocks.mock(|when, then| {
        when.get().path("/health");
        then.text("healthy!");
    });
    // Alternatively, Mock::new() and mocks.insert(mock)

    // Create mock server with the mock set
    let mut server = MockServer::new("example").with_mocks(mocks);
    server.run().await?;
```

Or, you can create a mock server with a default empty mock set and register mocks directly to the server:
```rust
    // Create mock server
    let mut server = MockServer::new("example");
    server.run().await?;

    // Build and insert a mock to the server's mock set
    server.mock(|when, then| {
        when.get().path("/health");
        then.text("healthy!");
    });
    // Alternatively, use Mock::new() and server.mocks.insert(mock)
```


