# Mock Builder

"Mock builder" refers to a closure with `When` and `Then` parameters used to build the request match conditions and response, respectively. 

```rust
    // A mock that returns the text "yo!" to any request
    let mock = Mock::new(|when, then| {
        when.any(); // builds a set of request match conditions
        then.text("yo!"); // builds the response to return when conditions are matched
    })
```

Together, they build a `Mock`, which consists of a set of request match conditions, a response, and a priority:

```rust
pub struct Mock {
    /// Mock ID.
    pub id: Uuid,
    /// A set of request match conditions.
    pub matchers: Vec<Arc<dyn Matcher>>,
    /// A mock response.
    pub response: Response,
    /// Priority.
    pub priority: u8, // defaults to 5 (more on this later)
    /// Match counter.
    pub match_count: AtomicUsize,
}
```

Since `when` and `then` are just variables of types `When` and `Then`, you can name them however you'd like, e.g. the following also works.

```rust
    // A mock that returns the text "index" to get requests on the / endpoint
    let mock = Mock::new(|req, res| {
        req.get().path("/");
        res.text("index");
    })
```

We experimented with several different APIs and found this closure-builder pattern to feel the most ergonomic and nice to use.

The mock builder closure is exposed via 3 methods, allowing flexible usage patterns:

1. `Mock::new(|when, then|...)` to build a standalone mock
2. `MockSet::mock(|when, then|...)` shorthand to build a mock and insert it into the mock set
3. `MockServer::mock(|when, then|...)` shorthand to build a mock and insert it into the server's mock set