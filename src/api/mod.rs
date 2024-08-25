pub mod file;
pub mod headers;

use crate::runtime_config::RuntimeConfig;

pub struct HirataStoreApi {
  pub hirata_store_api_url: String,
}

impl Default for HirataStoreApi {
  fn default() -> Self {
    Self::new()
  }
}

impl HirataStoreApi {
  pub fn new() -> Self {
    HirataStoreApi {
      hirata_store_api_url: RuntimeConfig::global().hirata_store_api_url.clone(),
    }
  }
}
