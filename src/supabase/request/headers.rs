use reqwest::header::{HeaderMap, HeaderValue};

use crate::runtime_config::RuntimeConfig;
use crate::supabase::request::{Headers, HeadersTypes};

impl Default for Headers {
  fn default() -> Self {
    Self::new()
  }
}

impl Headers {
  pub fn new() -> Self {
    let config = RuntimeConfig::global();

    let mut headers = HeaderMap::new();
    headers.insert(
      HeadersTypes::ApiKey.as_str(),
      HeaderValue::from_str(&config.supabase_key).unwrap(),
    );
    headers.insert(
      HeadersTypes::Authorization.as_str(),
      HeaderValue::from_str(&format!("Bearer {}", config.supabase_key)).unwrap(),
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

  pub fn set_user_authorization(&mut self, token: &str) -> &mut Self {
    self.headers.insert(
      HeadersTypes::Authorization.as_str(),
      HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    self
  }

  pub fn into_header_map(self) -> HeaderMap {
    self.headers
  }
}
