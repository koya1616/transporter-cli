use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

use crate::runtime_config::RuntimeConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
  pub access_token: String,
  pub refresh_token: String,
}

impl Default for Auth {
  fn default() -> Self {
    Self::new()
  }
}

impl Auth {
  pub fn new() -> Self {
    Auth {
      access_token: "".to_string(),
      refresh_token: "".to_string(),
    }
  }

  pub fn save_data(&self) -> std::io::Result<()> {
    let json = serde_json::to_string(self)?;
    let mut file = File::create(RuntimeConfig::global().config_folder.join("auth.json"))?;
    file.write_all(json.as_bytes())?;
    Ok(())
  }

  pub fn read_data(&self) -> std::io::Result<Auth> {
    let mut file = File::open(RuntimeConfig::global().config_folder.join("auth.json"))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let user: Self = serde_json::from_str(&contents)?;
    Ok(user)
  }
}
