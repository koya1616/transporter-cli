use std::sync::OnceLock;

pub struct RuntimeConfig {
  pub supabase_url: String,
  pub supabase_key: String,
}

impl RuntimeConfig {
  pub fn global() -> &'static RuntimeConfig {
    static INSTANCE: OnceLock<RuntimeConfig> = OnceLock::new();
    INSTANCE.get_or_init(|| {
      RuntimeConfig {
        supabase_url: "https://awwcgrfqoklqbgzzkawx.supabase.co".to_string(),
        supabase_key: "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImF3d2NncmZxb2tscWJnenprYXd4Iiwicm9sZSI6ImFub24iLCJpYXQiOjE3MjQwNzU0ODMsImV4cCI6MjAzOTY1MTQ4M30.AP31pL-Ed94gkM32ttFFiTUwVwZri5KDkXoWgFV022U".to_string(),
      }
    })
  }
}
