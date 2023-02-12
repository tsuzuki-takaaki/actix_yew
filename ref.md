ref: 「実践Rustプログラミング入門」

## prepared statement
- https://docs.rs/rusqlite/latest/rusqlite/struct.Statement.html#method.query_map
- https://docs.rs/hdbconnect/latest/hdbconnect/struct.PreparedStatement.html
- query_map: https://docs.rs/rusqlite/0.6.0/rusqlite/struct.Statement.html#method.query_map
  - Executes the prepared statement and maps a function over the resulting rows.Unlike the iterator produced by query, the returned iterator does not expose the possibility for accessing stale rows.

## serialize, deserialize(**most important**)
- https://serde.rs/
### serde_json
- serialize(rust -> network)
  - https://docs.rs/serde_json/latest/serde_json/#creating-json-by-serializing-data-structures
- deserialize(network -> rust: parse)
  - https://docs.rs/serde_json/latest/serde_json/#parsing-json-as-strongly-typed-data-structures
  