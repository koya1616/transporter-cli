pub mod api;
pub mod models;
pub mod runtime_config;
pub mod supabase;

use serde_json::Value;
use std::env;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::api::HirataStoreApi;
use crate::models::local_storage::Auth;
use crate::supabase::auth::SupabaseAuth;

fn read_single_line() -> io::Result<String> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}

fn handle_no_args() {
  println!("Welcome to the CLI app!");
  println!("Available commands:");
  println!("  signup - Create a new account");
  println!("  login  - Log in to your account");
  println!("  upload - Upload a file to the Hirata Store");
}

async fn handle_signup(email: &str, password: &str) {
  let auth = SupabaseAuth::new();
  match auth.signup(email, password).await {
    Ok(response) => match response.status().is_success() {
      true => println!("Signup successful!"),
      false => println!("Signup failed: {:?}", response.text().await.unwrap()),
    },
    Err(error) => {
      println!("Signup error: {:?}", error);
    }
  }
}

async fn handle_login(email: &str, password: &str) {
  let auth = SupabaseAuth::new();
  match auth.login(email, password).await {
    Ok(response) => match response.status().is_success() {
      true => {
        let response_text = response.text().await.unwrap();
        let json: Value = serde_json::from_str(&response_text).unwrap();

        if let (Some(access_token), Some(refresh_token)) =
          (json["access_token"].as_str(), json["refresh_token"].as_str())
        {
          let auth = Auth {
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
          };
          auth.save_data().unwrap();

          println!("{} でログインしました", email);
        }
      }
      false => println!("Login failed: {:?}", response.text().await.unwrap()),
    },
    Err(error) => {
      println!("Login error: {:?}", error);
    }
  }
}

async fn handle_upload(file_path: &str) {
  let hirata_store_api = HirataStoreApi::new();
  match hirata_store_api.upload(file_path).await {
    Ok(response) => match response.status().is_success() {
      true => println!("Upload successful!"),
      false => println!("Upload failed: {:?}", response.text().await.unwrap()),
    },
    Err(error) => {
      println!("Upload error: {:?}", error);
    }
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args: Vec<String> = env::args().collect();
  match args.len() {
    1 => handle_no_args(),
    2 => match args[1].as_str() {
      "signup" => {
        print!("メールアドレスを入力してください: ");
        io::stdout().flush()?;
        let email = read_single_line()?;

        print!("パスワードを入力してください: ");
        io::stdout().flush()?;
        let password = read_single_line()?;

        handle_signup(&email, &password).await;
      }
      "login" => {
        print!("メールアドレスを入力してください: ");
        io::stdout().flush()?;
        let email = read_single_line()?;

        print!("パスワードを入力してください: ");
        io::stdout().flush()?;
        let password = read_single_line()?;
        handle_login(&email, &password).await;
      }
      "upload" => println!("Too few arguments. Use 'upload <file_path>'."),
      _ => println!("Unknown command. Use 'signup' or 'login'."),
    },
    3 => match args[1].as_str() {
      "upload" => {
        let file_path = &args[2];
        let path = Path::new(file_path);
        match path.exists() {
          true => handle_upload(file_path).await,
          false => println!("指定されたファイルが存在しません: {}", file_path),
        }
      }
      _ => println!("Invalid command. For upload, use 'upload <file_path>'."),
    },
    _ => println!("Too many arguments. Use no arguments, 'signup', or 'login'."),
  }

  Ok(())
}
