#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = liUtilsFetcher)]
  pub type FetcherError;

  #[wasm_bindgen(constructor, js_namespace = liUtilsFetcher)]
  pub fn new(message: &str) -> FetcherError;
}
