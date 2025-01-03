use literateink_wasm as wasm;

// the method will be renamed `retrieveCas` in the generated bindings
#[wasm::export]
pub fn retrieve_cas() {}

// the method will be still called `update` in the generated bindings
#[wasm::export]
pub fn update() {}

#[wasm::export]
pub enum SomeEnum {
  A,
  B,
  C,
}

#[wasm::export]
pub struct Session {
  instance_url: String,
  php_sess_id: String,
  fetcher: js_sys::Function,
}

#[wasm::export]
impl Session {
  #[wasm_bindgen(constructor)]
  pub fn new(instance_url: String, php_sess_id: String, fetcher: js_sys::Function) -> Self {
    Self {
      instance_url,
      php_sess_id,
      fetcher,
    }
  }

  #[wasm_bindgen(getter)]
  pub fn instance_url(&self) -> String {
    self.instance_url.clone()
  }

  #[wasm_bindgen(getter)]
  pub fn php_sess_id(&self) -> String {
    self.php_sess_id.clone()
  }
}

impl Session {
  pub fn fetcher(&self) -> &js_sys::Function {
    &self.fetcher
  }
}

fn main() {
  println!("Hello, world!");
}
