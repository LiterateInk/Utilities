use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer};

pub use http::{Method, HeaderName, HeaderMap};
pub use url::Url;

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch (request: Request) -> Response {
  use reqwest::{Client, redirect::Policy};

  let client = if !request.follow {
    Client::builder()
      .redirect(Policy::none())
      .build().unwrap()
  }
  else {
    Client::new()
  };

  let response = client.request(request.method, request.url)
    .headers(request.headers)
    .body(request.body.unwrap_or_default())
    .send().await.unwrap();

  let status = response.status().as_u16();

  let headers = response.headers().iter().map(|(key, value)| {
    let key = key.as_str().to_string();
    let value = value.to_str().unwrap_or_default().to_string();

    (key, value)
  }).collect();

  let bytes = response.bytes().await.unwrap();
  let bytes = bytes.to_vec();

  Response {
    status,
    headers,
    bytes,
  }
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch (request: Request, fetcher: &js_sys::Function) -> Response {
  use wasm_bindgen_futures::JsFuture;
  use wasm_bindgen::JsValue;
  use js_sys::Promise;

  let request = serde_wasm_bindgen::to_value(&request).unwrap();
  let response = Promise::from(fetcher.call1(&JsValue::NULL, &request).unwrap());
  let response = JsFuture::from(response).await.unwrap();
  serde_wasm_bindgen::from_value::<Response>(response).unwrap()
}

pub struct Request {
  pub url: Url,
  pub method: Method,
  pub body: Option<String>,
  pub headers: HeaderMap,
  /// Whether we should follow redirects or not.
  pub follow: bool,
}

impl Serialize for Request {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
    use serde::ser::SerializeStruct;

    let mut state = serializer.serialize_struct("Request", 4)?;
    state.serialize_field("url", &self.url.as_str())?;
    state.serialize_field("method", &self.method.as_str())?;

    let headers: Vec<(String, String)> = self.headers.iter().map(|(key, value)| {
      (key.as_str().to_string(), value.to_str().unwrap_or_default().to_string())
    }).collect();

    state.serialize_field("headers", &headers)?;

    if let Some(body) = &self.body {
      state.serialize_field("body", body)?;
    }

    state.serialize_field("follow", &self.follow)?;

    state.end()
  }
}

#[derive(Deserialize)]
pub struct Response {
  pub status: u16,
  /// Headers are represented as a list of key-value pairs
  /// where the key is the header name (always lowercase) and the value is the header value.
  headers: Vec<(String, String)>,
  #[serde(with = "serde_bytes")]
  pub bytes: Vec<u8>,
}

impl Response {
  pub fn text(&self) -> String {
    String::from_utf8_lossy(&self.bytes).to_string()
  }
  pub fn headers(&self) -> HeaderMap {
    let mut headers = HeaderMap::new();

    for (key, value) in &self.headers {
      let key = HeaderName::from_str(key.as_str()).unwrap();
      let value = value.parse().unwrap();

      headers.insert(key, value);
    }

    headers
  }
}
