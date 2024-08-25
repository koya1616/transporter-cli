use std::env;
use std::path::PathBuf;
use std::sync::OnceLock;

pub struct RuntimeConfig {
  pub supabase_url: String,
  pub supabase_key: String,
  pub hirata_store_api_url: String,
  pub config_folder: PathBuf,
}

impl RuntimeConfig {
  fn get_config_file_path() -> PathBuf {
    env::var("HOME")
      .map(PathBuf::from)
      .unwrap_or_else(|_| PathBuf::from("."))
      .join(".hirata_store")
  }

  pub fn global() -> &'static RuntimeConfig {
    static INSTANCE: OnceLock<RuntimeConfig> = OnceLock::new();
    INSTANCE.get_or_init(|| {
      RuntimeConfig {
        supabase_url: "https://awwcgrfqoklqbgzzkawx.supabase.co".to_string(),
        supabase_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImF3d2NncmZxb2tscWJnenprYXd4Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjQwNzU0ODMsImV4cCI6MjAzOTY1MTQ4M30.AP31pL-Ed94gkM32ttFFiTUwVwZri5KDkXoWgFV022U".to_string(),
        hirata_store_api_url: "https://minimal-tish-hirata-store-eaeab429.koyeb.app".to_string(),
        config_folder: Self::get_config_file_path(),
      }
    })
  }
}
