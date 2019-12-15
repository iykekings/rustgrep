use std::env;
use std::process::Command;
//use std::io::{self, Write};
//use std::env::Args;

fn main() {
    let options: Vec<String> = env::args().collect();
    let output = Command::new("ls").output().expect("Couldn't run command").stdout;
    let list = String::from_utf8(output).unwrap();
    let result = list.lines().collect::<Vec<&str>>();
    println!("{:?}", options);
    println!("{:#?}", result);
}
