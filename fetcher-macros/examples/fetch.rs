use fetcher::{fetch, Request, Method};

#[tokio::main]
async fn main () {
  let request = Request {
    url: "https://example.com".parse().unwrap(),
    method: Method::GET,
    headers: Default::default(),
    body: None,
    follow: false,
  };

  let response = fetch!(request);
  println!("Received {} !", response.status);
}
