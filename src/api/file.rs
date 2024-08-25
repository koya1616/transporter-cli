use reqwest::multipart::{Form, Part};
use reqwest::{Client, Error, Response};
use std::fs::File;
use std::io::Read;

use crate::api::headers::Headers;
use crate::api::HirataStoreApi;

impl HirataStoreApi {
  pub async fn upload(&self, file_path: &str) -> Result<Response, Error> {
    let url: String = format!("{}/upload", self.hirata_store_api_url);

    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let form = Form::new()
      .text("name", "testfromcli.ipa")
      .part("file", Part::bytes(buffer).mime_str("application/octet-stream")?);

    let response: Response = Client::new()
      .post(&url)
      .headers(Headers::new().into_header_map())
      .multipart(form)
      .send()
      .await?;

    Ok(response)
  }
}
