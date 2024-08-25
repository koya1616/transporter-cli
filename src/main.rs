pub mod runtime_config;
pub mod supabase;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

use crate::supabase::auth::SupabaseAuth;

fn read_single_line() -> io::Result<String> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
  access_token: String,
  refresh_token: String,
}

fn save_data(filename: &str, user: &User) -> std::io::Result<()> {
  let json = serde_json::to_string(user)?;
  let mut file = File::create(filename)?;
  file.write_all(json.as_bytes())?;
  Ok(())
}

fn read_data(filename: &str) -> std::io::Result<User> {
  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let user: User = serde_json::from_str(&contents)?;
  Ok(user)
}

fn handle_no_args() {
  println!("Welcome to the CLI app!");
  println!("Available commands:");
  println!("  signup - Create a new account");
  println!("  login  - Log in to your account");
}

async fn handle_signup(email: String, password: String) {
  let auth = SupabaseAuth::new();
  match auth.signup(email, password).await {
    Ok(response) => {
      println!("Signup response: {:?}", response.text().await.unwrap());
    }
    Err(error) => {
      println!("Signup error: {:?}", error);
    }
  }
}

async fn handle_login(email: String, password: String) {
  let auth = SupabaseAuth::new();
  match auth.login(email, password).await {
    Ok(response) => {
      let response_text = response.text().await.unwrap();
      let json: Value = serde_json::from_str(&response_text).unwrap();

      if let (Some(access_token), Some(refresh_token)) = (json["access_token"].as_str(), json["refresh_token"].as_str())
      {
        let user = User {
          access_token: access_token.to_string(),
          refresh_token: refresh_token.to_string(),
        };

        save_data("user.json", &user).unwrap();

        let loaded_user = read_data("user.json").unwrap();
        println!("Loaded user: {:?}", loaded_user);
      }
    }
    Err(error) => {
      println!("Login error: {:?}", error);
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

        handle_signup(email, password).await;
      }
      "login" => {
        print!("メールアドレスを入力してください: ");
        io::stdout().flush()?;
        let email = read_single_line()?;

        print!("パスワードを入力してください: ");
        io::stdout().flush()?;
        let password = read_single_line()?;
        handle_login(email, password).await;
      }
      _ => println!("Unknown command. Use 'signup' or 'login'."),
    },
    _ => println!("Too many arguments. Use no arguments, 'signup', or 'login'."),
  }

  Ok(())
}
