# Then

`Then` is a builder used to build responses.

### Body methods:
- `body()` *(primary)*
- `empty()`
- `bytes()`
- `bytes_stream()`
- `text()`
- `text_stream()`
- `json()`
- `json_lines_stream()`
- `pb()`
- `pb_stream()`

### Headers method:
- `headers()`

### Status methods:
- `status()` *(primary)*
- `message()`
- `error()`
- `ok()`
- `bad_request()`
- `unauthorized()`
- `forbidden()`
- `not_found()`
- `unsupported_media_type()`
- `unprocessable_content()`
- `internal_server_error()`
- `not_implemented()`
- `bad_gateway()`
- `service_unavailable()`
- `gateway_timeout()`