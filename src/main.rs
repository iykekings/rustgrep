use rustgrep::{run, Config};

use std::{env, process};

fn main() {
    let config = Config::new(env::args());
    let result = match config {
        Ok(conf) => run(conf),
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };
    for line in result {
        println!("{}", line)
    }
}
