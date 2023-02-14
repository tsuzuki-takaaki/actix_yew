ref: 「実践Rustプログラミング入門」
ref: https://tech.isid.co.jp/entry/2022/10/11/%E3%80%90Yew%E3%80%91Rust%E3%81%A7%E3%83%95%E3%83%AD%E3%83%B3%E3%83%88%E3%82%A8%E3%83%B3%E3%83%89%E9%96%8B%E7%99%BA_-_Rust%E3%81%AE%E3%83%9E%E3%82%AF%E3%83%AD%E3%82%92%E7%B4%90%E8%A7%A3%E3%81%8F_-

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
## yew
- https://yew.rs/docs/tutorial
- csr(client side rendering): https://docs.rs/yew/latest/yew/#supported-features
## gloo_net: HTTP request library for WASM app
- https://docs.rs/gloo-net/latest/gloo_net/
## wasm_bindgen_futures
- convert between javascript **Promise** to Rust **Future**
- https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen_futures/

## Debugging(yew)
- wasm-logger: https://yew.rs/docs/more/debugging
- gloo_console
  - https://yew.rs/docs/more/debugging#gloo-console
  - https://docs.rs/gloo-console/0.2.3/gloo_console/

## CORS
- actix-cors: https://docs.rs/actix-cors/latest/actix_cors/index.html
  - https://github.com/security-union/yew-actix-template/blob/main/actix-api/src/main.rs#L160
  - **check dependencies**

## classes!
- https://yew.rs/ja/docs/concepts/html/classes

## yew tutorial
- counter
  - https://youtu.be/KmOeFrwz8BM

## function_component macro(attribute)
- https://docs.rs/yew-macro/latest/src/yew_macro/lib.rs.html#133
- about derive
  - https://doc.rust-lang.org/rust-by-example/trait/derive.html#derive

## struct component
- https://yew.rs/docs/concepts/function-components#two-flavours-of-yew-components
> You are currently reading about function components - the recommended way to write components when starting with Yew and when writing simple presentation logic.

>There is a **more advanced, but less accessible**, way to write components - **Struct components**. They allow very detailed control, though you will not need that level of detail most of the time.

## enum type alias
- https://doc.rust-lang.org/rust-by-example/custom_types/enum.html#type-aliases
