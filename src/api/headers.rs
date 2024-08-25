use reqwest::header::{HeaderMap, HeaderValue};

use crate::Auth;

enum HeadersTypes {
  Authorization,
  ContentType,
}

pub struct Headers {
  pub headers: HeaderMap,
}

impl Default for Headers {
  fn default() -> Self {
    Self::new()
  }
}

impl Headers {
  pub fn new() -> Self {
    let auth = Auth::new().read_data().unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(
      HeadersTypes::Authorization.as_str(),
      HeaderValue::from_str(&auth.access_token).unwrap(),
    );
    Headers { headers }
  }

  pub fn json(&mut self) -> &mut Self {
    self.headers.insert(
      HeadersTypes::ContentType.as_str(),
      HeaderValue::from_static("application/json"),
    );
    self
  }

  pub fn into_header_map(self) -> HeaderMap {
    self.headers
  }
}

impl HeadersTypes {
  fn as_str(&self) -> &str {
    match self {
      HeadersTypes::Authorization => "Authorization",
      HeadersTypes::ContentType => "Content-Type",
    }
  }
}
