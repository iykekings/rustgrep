![](https://github.com/iykekings/rustgrep/workflows/Test/badge.svg)


## Simple grep tool with Rust

> Install

  Clone this repo, `cd rustgrep`

  ```sh
    cargo install --path .
  ```

> Run

 If you followed the install step above - just Run
 
  ```sh 
    <cmd> | rustgrep <query>
  ```

  Example:
  ```sh
    ifconfig | rustgrep net
  ```

  Or
  ```sh
    <cmd> | cargo run <query>
  ```

  > Test

  ```sh
    cargo test
  ```