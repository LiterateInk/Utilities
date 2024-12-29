#[derive(thiserror::Error, Debug, wasm::Error)]
pub enum Error {
  #[error("no redirection was made, make sure the instance URL is correct")]
  InvalidRedirection(),
}

fn main() {
  println!("Hello, world!");
}
