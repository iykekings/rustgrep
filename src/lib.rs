use std::process::{self, Command};

pub struct Config {
  pub command: String,
  pub query: String,
}

impl Config {
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    args.next();
    let command = match args.next() {
      Some(res) => res,
      None => return Err("Didn't get command"),
    };
    let query = match args.next() {
      Some(res) => res,
      None => return Err("Didn't get query"),
    };

    Ok(Config { command, query })
  }
}

fn search(query: String, contents: String) -> Vec<String> {
  contents
    .lines()
    .filter(|line| line.contains(query.as_str()))
    .map(|m| String::from(m))
    .collect()
}

pub fn run(config: Config) -> Vec<String> {
  let output = Command::new(config.command).output().unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  let texts = String::from_utf8(output.stdout).unwrap();
  search(config.query, texts)
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
