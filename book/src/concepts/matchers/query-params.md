# Query Params

## Query Params

Matches a request by query params. Returns `true` if the request query params are *equal to* the query params.

### `When` method:
#### `query_params(params)`
Query params. `params` is an iterator of key-value pairs.

## Query Param

Matches a request by query param. Returns `true` if the request contains a query param *equal to* the query param.

### `When` method:
#### `query_param(key, value)`
Query param. `key` and `value` are types implementing `Into<String>`.

## Query Param Exists

Matches a request by query param exists. Returns `true` if the request contains a query param with the query key.

### `When` method:
#### `query_param_exists(key)`
Query param exists. `key` is a type implementing `Into<String>`.