# `wasm`

## Installation

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm = { git = "https://github.com/LiterateInk/Utilities" }
wasm-bindgen = "0.2" # required for the `wasm_bindgen::prelude::wasm_bindgen` macro
js-sys = "0.3" # required for the `js_sys::Function` type
```

## Usage

```rust
#[cfg_attr(target_arch = "wasm32", wasm::api_method)]
pub async fn fetch_github(something: String) -> String {
  "hello world".to_string()
}
```

A `fetcher` parameter is automatically added to the function signature.
See the [`fetcher` module](../fetcher) for more information.
