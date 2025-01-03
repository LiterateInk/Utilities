use literateink_wasm as wasm;

#[derive(thiserror::Error, wasm::Error, Debug)]
pub enum Error {
  #[error("no redirection was made, make sure the instance URL is correct")]
  InvalidRedirection(),
  #[error("the response was not successful, status code: {0}")]
  UnsuccessfulResponse(u16),
}

fn main() {
  println!("Hello, world!");
}
