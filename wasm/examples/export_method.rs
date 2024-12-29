#[wasm::api_method(retrieveCAS)]
pub fn retrieve_cas() {
  // a `fetcher` variable is available
  // if the target architecture is `wasm32`
}

// the method will be still called `update` in the generated bindings
#[wasm::api_method]
pub fn update() {
  // a `fetcher` variable is available
  // if the target architecture is `wasm32`
}

fn main() {
  println!("Hello, world!");
}
