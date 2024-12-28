use fetcher::{fetch, Request};

#[tokio::main]
async fn main () {
  let request = Request {
    url: "https://example.com".parse().unwrap(),
    method: fetcher::Method::GET,
    headers: Default::default(),
    body: None,
    follow: None,
  };

  let response = fetch!(request);
  println!("Received {} !", response.status);
}
