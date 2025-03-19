# Defining Mocks

Mocks are defined in Rust using a simple, ergonomic builder-like API. See [Mock Builder](./concepts/mock-builder.md) for additional details.

You can define your mocks first, then create a mock server with your mock set.
```rust
    // Build and insert mocks using MockSet::mock()
    let mut mocks = MockSet::new();
    mocks.mock(|when, then| {
        when.get().path("/health");
        then.text("healthy!");
    });
    // Alternatively, use Mock::new() and MockSet::insert()

    // Create mock server with the mock set
    let mut server = MockServer::new("example").with_mocks(mocks);
    server.run().await?;
```

Alternatively, you can create a mock server with a default (empty) mock set and register mocks directly to the server.
```rust
    // Create mock server (with a default empty mock set)
    let mut server = MockServer::new("example");
    server.run().await?;

    // Register mocks to the server directly using MockServer::mock()
    server.mock(|when, then| {
        when.get().path("/health");
        then.text("healthy!");
    });
    // Alternatively, use Mock::new() and MockServer::mocks().insert()
```


