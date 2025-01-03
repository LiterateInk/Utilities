use fetcher::FetcherError;

#[derive(thiserror::Error, Debug)]
#[cfg_attr(target_arch = "wasm32", derive(literateink_wasm::Error))]
pub enum Error {
  #[error("no redirection was made, make sure the instance URL is correct")]
  InvalidRedirection(),
  #[error("the response was not successful, status code: {0}")]
  UnsuccessfulResponse(u16),
  #[error(transparent)]
  FetcherError(#[from] FetcherError),
}

fn main() {
  println!("Hello, world!");
}
