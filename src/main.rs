use std::io::{self, Write};

fn read_single_line() -> io::Result<String> {
  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  Ok(input.trim().to_string())
}

fn main() -> io::Result<()> {
  print!("何か入力してみて: ");
  io::stdout().flush()?;
  let line = read_single_line()?;
  println!("Nice: {}", line);
  Ok(())
}
