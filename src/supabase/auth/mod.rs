pub mod user;

impl Default for SupabaseAuth {
  fn default() -> Self {
    Self::new()
  }
}

pub struct SupabaseAuth {
  pub supabase_url: String,
}
