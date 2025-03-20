# Any

## Any

Matches any request. Should not be combined with other matchers.

### `When` method
#### `any()`
Matches any request.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.any();
    then.ok();
})
```