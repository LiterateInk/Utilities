# `wasm`

## Installation

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm = { package = "literateink-wasm", version = "0.1" }
wasm-bindgen = "0.2" # required for the `wasm_bindgen::prelude::wasm_bindgen` macro
js-sys = "0.3" # required for the `js_sys::Function` type
```
