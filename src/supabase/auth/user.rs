use reqwest::{Client, Error, Response};
use serde_json::json;

use crate::runtime_config::RuntimeConfig;
use crate::supabase::auth::SupabaseAuth;
use crate::supabase::request::Headers;

impl SupabaseAuth {
  pub fn new() -> Self {
    SupabaseAuth {
      supabase_url: RuntimeConfig::global().supabase_url.clone(),
    }
  }

  pub async fn signup(&self, email: &str, password: &str) -> Result<Response, Error> {
    let url: String = format!("{}/auth/v1/signup", self.supabase_url);
    let mut headers = Headers::new();
    headers.json();

    let response: Response = Client::new()
      .post(&url)
      .headers(headers.into_header_map())
      .body(json!({ "email": email, "password": password }).to_string())
      .send()
      .await?;

    Ok(response)
  }

  pub async fn login(&self, email: &str, password: &str) -> Result<Response, Error> {
    let url: String = format!("{}/auth/v1/token?grant_type=password", self.supabase_url);
    let mut headers = Headers::new();
    headers.json();

    let response: Response = Client::new()
      .post(&url)
      .headers(headers.into_header_map())
      .body(json!({ "email": email, "password": password }).to_string())
      .send()
      .await?;

    Ok(response)
  }

  pub async fn get_user(&self, user_authorization: &str) -> Result<Response, Error> {
    let url: String = format!("{}/auth/v1/user", self.supabase_url);
    let mut headers = Headers::new();
    headers.set_user_authorization(user_authorization);

    let response: Response = Client::new()
      .get(&url)
      .headers(headers.into_header_map())
      .send()
      .await?;

    Ok(response)
  }
}
