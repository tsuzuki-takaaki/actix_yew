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

## Callbacks
- https://yew.rs/docs/advanced-topics/struct-components/callbacks

## Event Listener
- https://yew.rs/ja/docs/concepts/html/events

## use_state(yew)
- https://docs.rs/yew/latest/yew/functional/fn.use_state.html
- input value change like react
  - https://yew.rs/ja/docs/0.19.0/concepts/function-components/pre-defined-hooks#example-1
- debug
  - https://crates.io/crates/wasm-logger
```rust
  log::debug!("{}", state);
```
  - console上に表示される

## web_sys
- https://yew.rs/docs/concepts/basic-web-technologies/web-sys

## target_unchecked_into()
- https://docs.rs/yew/latest/yew/html/trait.TargetCast.html#method.target_unchecked_into

## creating node
- https://github.com/jetli/rust-yew-realworld-example-app/blob/77b145bf2e3ba38e43131720f976c6974ded8fb7/crates/conduit-wasm/src/routes/article/mod.rs#L90

## oninput, onchange
- https://github.com/yewstack/yew/issues/233

## InputEvent
- web_sys: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.InputEvent.html
- MDN: https://developer.mozilla.org/en-US/docs/Web/API/InputEvent
- inputEvent.data => get input event
- **web_sys's InputEvent is almost same with MDN**

## Event
- web_sys: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Event.html
- MDN: https://developer.mozilla.org/en-US/docs/Web/API/Event

## onchange vs oninput
- onchange is the event that is triggerd after one event is done
- oninput is the event that is triggered when typing into input area
- so, you should use **oninput** if you would like to do like **react onchange**
```rust
  // ex
  let input_value = use_state(|| String::from(""));

  let handle_change = {
    let input_value = input_value.clone();
    Callback::from(move |e: web_sys::InputEvent| {
      let new_input_value: HtmlInputElement = e.target_unchecked_into();
      input_value.set(new_input_value.value());
    })
  };
```
- ref: https://github.com/yewstack/yew/issues/233

## UseStateHandle
- UseStateHandle 構造体は set メソッドにより値を設定します。* 演算子により参照を外すことにより UseStateHandle 構造体より現在の状態を取得できます。
- the value of use_state is wrapped by UseStateHandle.
- **if you wanna access to the value wrapped UseStateHandle, you can use *(dereference).**

## yew with css
- https://github.com/yewstack/yew/discussions/2003

## trunk with css
- https://trunkrs.dev/assets/
- if you wanna make directory for styles, you add ```data-trunk``` to link tag like below
```html
  <link data-trunk rel="css" href="styles/index.css" />
```

## if you wanna post new post
```curl -X POST -d '{"title": "This is a tweet", "body": "hello world"}' -H "Content-type: application/json" http://localhost:3000/posts```

# More
- web_sys: https://rustwasm.github.io/wasm-bindgen/api/web_sys/
