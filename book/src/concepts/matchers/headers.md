# Headers

## Headers

Matches a request by headers. Returns `true` if the request headers are a *superset* of the headers, i.e. contains *at least* all of the headers.

### `When` method:
- `headers()`

## Headers Exact

Matches a request by exact headers. Returns `true` if the request headers are *equal to* the headers.

### `When` method:
- `headers_exact()`

## Header

Matches a request by header. Returns `true` if the request contains a header *equal to* the header.

### `When` method:
- `header()`

## Header Exists

Matches a request by header exists. Returns `true` if the request contains a header with the header name.

### `When` method:
- `header_exists()`
