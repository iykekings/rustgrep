use rustgrep::{get_stdin, search, Config};
use std::{env, process};

fn main() {
    let config = Config::new(env::args());
    let content = get_stdin();
    if content.len() < 1 {
        eprintln!("No result - Empty content provided");
        process::exit(1);
    }
    let result = match config {
        Ok(conf) => search(conf.query, content),
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };
    for line in result {
        println!("{}", line)
    }
}
