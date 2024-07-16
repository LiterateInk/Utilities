use futures::{future::LocalBoxFuture, FutureExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
  pub status: u16,
  pub content: String,
  pub headers: Vec<(String, String)>
}

#[derive(Serialize, Deserialize)]
pub struct Request {
  pub url: String,
  pub method: String,
  pub content: Option<String>,
  pub headers: Vec<(String, String)>
}

/// Wraps a JS function (a fetcher) that fetches a URL and returns a Future that resolves to a Response.
/// Should only be used in a `target_arch = "wasm32"` context.
pub fn wasm_wrap_fetcher(f: &js_sys::Function) -> impl Fn(Request) -> LocalBoxFuture<'static, Result<Response, String>> {
  use js_sys::Promise;
  use wasm_bindgen::JsValue;
  use wasm_bindgen_futures::JsFuture;

  let f = f.clone(); // Clone the function to avoid lifetime issues
  move |req: Request| {
    let this = JsValue::null();
    let fetch_promise: Promise = f.call1(&this, &serde_wasm_bindgen::to_value(&req).unwrap()).unwrap().into();
    
    async move {
      let js_future = JsFuture::from(fetch_promise);
      match js_future.await {
        Ok(result) => Ok(serde_wasm_bindgen::from_value(result).unwrap()),
        Err(err) => Err(err.as_string().unwrap_or_else(|| "Unknown error in 'wasm_wrap_fetcher', your request was probably badly formatted.".to_string())),
      }
    }.boxed_local()
  }
}

#[cfg(not(target_arch = "wasm32"))]
/// Uses reqwest to fetch the content of a URL.
pub fn reqwest_fetcher(req: Request) -> LocalBoxFuture<'static, Result<Response, String>> {
  use reqwest::{ Client, header::{HeaderMap, HeaderName, HeaderValue }};
  use std::str::FromStr;
  
  // Create a reqwest client
  let client = Client::new();

  // Prepare headers as a HashMap
  let mut headers = HeaderMap::new();
  for (key, value) in req.headers {
    headers.insert(
      HeaderName::from_str(&key).unwrap(),
      HeaderValue::from_str(&value).unwrap()
    );
  }

  // Build the request
  let builder = match req.method.as_str() {
    "GET" => client.get(&req.url),
    "POST" => client.post(&req.url),
    // Never used in our libraries...
    // "PUT" => client.put(&req.url),
    // "DELETE" => client.delete(&req.url),
    _ => panic!("Unsupported HTTP method: {}", req.method),
  };

  // Add headers
  let builder = builder.headers(headers);

  // Set body if content is present
  let builder = if let Some(content) = req.content {
    builder.body(content)
  } else {
    builder
  };

  async move {
    match builder.send().await {
      Ok(response) => {
        let status = response.status();
        let headers = response.headers().clone();

        match response.text().await {
          Ok(text) => {
            Ok(Response {
              headers: headers.iter().map(|(k, v)| (
                k.as_str().to_string(),
                v.to_str().unwrap().to_string()
              )).collect(),
              status: status.as_u16(),
              content: text
            })
          },
          Err(err) => Err(err.to_string()),
        }
      }
      Err(err) => Err(err.to_string()),
    }
  }.boxed_local()
}
