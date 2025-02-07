pub use http::{HeaderMap, HeaderName, Method};
pub use url::Url;

#[cfg(target_arch = "wasm32")]
use serde::{Deserialize, Serialize, Serializer};

#[cfg(target_arch = "wasm32")]
mod bridge;

#[derive(thiserror::Error, Debug)]
pub struct FetcherError(String);

impl std::fmt::Display for FetcherError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[cfg(target_arch = "wasm32")]
impl From<FetcherError> for wasm_bindgen::JsValue {
  fn from(error: FetcherError) -> Self {
    bridge::FetcherError::new(&error.0).into()
  }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reqwest::Error> for FetcherError {
  fn from(error: reqwest::Error) -> Self {
    FetcherError(error.to_string())
  }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch(request: Request) -> Result<Response, FetcherError> {
  use reqwest::{redirect::Policy, Client};

  let client = if !request.follow {
    Client::builder().redirect(Policy::none()).build()?
  } else {
    Client::new()
  };

  let response = client
    .request(request.method, request.url)
    .headers(request.headers)
    .body(request.body.unwrap_or_default())
    .send()
    .await?;

  let status = response.status().as_u16();
  let headers = response.headers().clone();

  let bytes = response.bytes().await?;
  let bytes = bytes.to_vec();

  Ok(Response {
    status,
    headers,
    bytes,
  })
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch(request: Request, fetcher: &js_sys::Function) -> Result<Response, FetcherError> {
  use js_sys::Promise;
  use std::str::FromStr;
  use wasm_bindgen::JsValue;
  use wasm_bindgen_futures::JsFuture;

  let request =
    serde_wasm_bindgen::to_value(&request).map_err(|err| FetcherError(err.to_string()))?;

  let response = Promise::from(fetcher.call1(&JsValue::NULL, &request).map_err(|err| {
    FetcherError(
      err
        .as_string()
        // try to get the error message from the error object
        .unwrap_or_else(|| js_sys::Error::from(err).to_string().into()),
    )
  })?);

  let response = JsFuture::from(response).await.map_err(|err| {
    FetcherError(
      err
        .as_string()
        // try to get the error message from the error object
        .unwrap_or_else(|| js_sys::Error::from(err).to_string().into()),
    )
  })?;

  let response = serde_wasm_bindgen::from_value::<ResponseWasm>(response)
    .map_err(|err| FetcherError(err.to_string()))?;

  let mut headers = HeaderMap::new();

  for (key, value) in response.headers {
    let key = HeaderName::from_str(key.as_str()).unwrap();
    let value = value.parse().unwrap();

    headers.append(key, value);
  }

  Ok(Response {
    status: response.status,
    headers,
    bytes: response.bytes,
  })
}

pub struct Request {
  pub url: Url,
  pub method: Method,
  pub body: Option<String>,
  pub headers: HeaderMap,
  /// Whether we should follow redirects or not.
  pub follow: bool,
}

#[cfg(target_arch = "wasm32")]
impl Serialize for Request {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    use serde::ser::SerializeStruct;

    let field_count = 4 + self.body.is_some() as usize;
    let mut state = serializer.serialize_struct("Request", field_count)?;
    state.serialize_field("url", &self.url.as_str())?;
    state.serialize_field("method", &self.method.as_str())?;

    let headers: Vec<(String, String)> = self
      .headers
      .iter()
      .map(|(key, value)| {
        (
          key.as_str().to_string(),
          value.to_str().unwrap_or_default().to_string(),
        )
      })
      .collect();

    state.serialize_field("headers", &headers)?;

    if let Some(body) = &self.body {
      state.serialize_field("body", body)?;
    }

    state.serialize_field("follow", &self.follow)?;

    state.end()
  }
}

#[cfg(target_arch = "wasm32")]
#[derive(Deserialize)]
struct ResponseWasm {
  pub status: u16,
  pub headers: Vec<(String, String)>,
  #[serde(with = "serde_bytes")]
  pub bytes: Vec<u8>,
}

pub struct Response {
  pub status: u16,
  pub headers: HeaderMap,
  pub bytes: Vec<u8>,
}

impl Response {
  pub fn text(&self) -> String {
    String::from_utf8_lossy(&self.bytes).to_string()
  }
}
