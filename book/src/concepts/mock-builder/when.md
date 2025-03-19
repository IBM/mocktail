# When

`When` is a builder used to build request match conditions.

### Method methods:
- `method()` *(primary)*
- `get()`
- `post()`
- `put()`
- `head()`
- `delete()`

### Path methods:
- `path()`
- `path_prefix()`

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

### Header methods:
- `headers()`
- `headers_exact()`
- `header()`
- `header_exists()`


### Query Param methods:
- `query_params()`
- `query_param()`
- `query_param_exists()`


### Other methods:
- `any()`
- `matcher()` *(for custom `Matcher` implementations)*