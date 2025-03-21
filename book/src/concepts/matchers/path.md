# Path

## Path

Matches a request by path.

### `When` method:

#### `path(path)`
Path.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.path("/path");
    then.ok();
})
```

## Path Prefix

Matches a request by path prefix. Returns `true` if the request path starts with prefix.

### `When` method:
#### `path_prefix(prefix)`
Path prefix.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.path_prefix("/p");
    then.ok();
})
```