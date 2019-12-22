use std::io::{self, Read};
use std::{process};

pub struct Config {
  pub query: String,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let query = match args.next() {
      Some(res) => res,
      None => return Err("Didn't get query"),
    };

    Ok(Config { query })
  }
}
pub fn get_stdin() -> String {
  let mut buffer = String::new();
  io::stdin()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|err| {
            eprintln!("Problem getting contents to search: {}", err);
            process::exit(1);
        });
  return buffer;
}

pub fn search(query: String, contents: String) -> Vec<String> {
  contents
    .lines()
    .filter(|line| line.contains(query.as_str()))
    .map(|m| String::from(m))
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search() {
    let query = String::from("saf");
    let contents = "Rust: safe\nsafe, fast, productive. Pick three.\nTrust me.";
    assert_eq!(
      vec!["Rust: safe", "safe, fast, productive. Pick three."],
      search(query, String::from(contents))
    );
  }
}
