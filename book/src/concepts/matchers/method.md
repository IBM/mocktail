# Method

## Method

Matches a request by HTTP method.

### `When` methods:
#### `method(method)` *(primary)*
HTTP method.

#### `get()`
HTTP GET method.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.get();
    then.ok();
})
```

#### `post()`
HTTP POST method.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.post();
    then.ok();
})
```

#### `put()`
HTTP PUT method.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.put();
    then.ok();
})
```

#### `head()`
HTTP HEAD method.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.head();
    then.ok();
})
```

#### `delete()`
HTTP DELETE method.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.delete();
    then.ok();
})
```