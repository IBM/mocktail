# Body

## Body

Matches a request by body.

### `When` methods:

#### `body(body)` *(primary)*

#### `empty()`
An empty body.

Example:
```rust
let mock = Mock::new(|when, then| {
    when.empty();
    then.ok();
})
```
#### `bytes(body)`
A raw bytes body. `body` is a type implementing `Into<Bytes>`.
```rust
let mock = Mock::new(|when, then| {
    when.bytes("hello".as_bytes());
    then.ok();
})
```
#### `bytes_stream(messages)`
A raw bytes streaming body. `messages` is an iterator of messages implementing `Into<Bytes>`.
```rust
let mock = Mock::new(|when, then| {
    when.bytes_stream([
        "msg1".as_bytes(), 
        "msg2".as_bytes(), 
        "msg3".as_bytes(),
    ]);
    then.ok();
})
```
#### `text(body)`
A text body. `body` is a type implementing `Into<String>`.
```rust
let mock = Mock::new(|when, then| {
    when.text("hello");
    then.ok();
})
```
#### `text_stream(messages)`
A text streaming body. `messages` is an iterator of `String` messages.
```rust
let mock = Mock::new(|when, then| {
    when.text_stream([
        "msg1", 
        "msg2", 
        "msg3"
    ]);
    then.ok();
})
```
#### `json(body)`
A json body. `body` is a type implementing `serde::Serialize`.
```rust
use serde_json::json;
let mock = Mock::new(|when, then| {
    when.json(json!({"message": "hello"}));
    then.ok();
})
```
#### `json_lines_stream(messages)`
A newline delimited json streaming body. `messages` is an iterator of messages implementing `serde::Serialize`.
```rust
use serde_json::json;
let mock = Mock::new(|when, then| {
    when.json_lines_stream([
        json!({"message": "msg1"}), 
        json!({"message": "msg2"}), 
        json!({"message": "msg3"}),
    ]);
    then.ok();
})
```
#### `pb(body)`
A protobuf body. `body` is a prost-generated type implementing `prost::Message`.
```rust
let mock = Mock::new(|when, then| {
    when.pb(ExampleMessage { message: "msg" });
    then.ok();
})
```
#### `pb_stream(messages)`
A protobuf streaming body. `messages` is an iterator of messages implementing `prost::Message`.
```rust
let mock = Mock::new(|when, then| {
    when.pb_stream([
        ExampleMessage { message: "msg1" }, 
        ExampleMessage { message: "msg2" }, 
        ExampleMessage { message: "msg3" },
    ]);
    then.ok();
})
```