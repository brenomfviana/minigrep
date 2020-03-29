//! This file is part of minigrep.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! Minigrep is a free software; you can redistribute it and/or modify it under
//! the terms of the MIT License.

use std::env;
use std::fs;
use std::process;

/// Search configuration values.
struct Config {
  query: String,
  filename: String,
}

impl Config {
  /// Gets the arguments, validates them and returns the search configuration.
  fn new(args: &[String]) -> Result<Config, &'static str> {
    // Check if the arguments are valid
    match args.len() {
      3 => { /* Right number of arguments. */ },
      0 => return Err("how did you do it?"),
      1 => return Err("you did not enter any arguments."),
      2 => return Err("you did not enter the filename."),
      _ => println!("WARNING: You entered more arguments than the necessary."),
    }
    // Return the search configuration
    let (query, filename) = (args[1].clone(), args[2].clone());
    Ok(Config{ query, filename })
  }
}

/// Perform file reading and searches for the phrase.
fn run(config: Config) {
  // Print arguments
  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);
  // Read the file
  let contents = fs::read_to_string(config.filename)
    .expect("ERROR: Something went wrong reading the file.");
  // Print the file content
  println!("With text:\n{}", contents);
}

fn main() {
  // Get arguments
  let args: Vec<String> = env::args().collect();
  // Get the search configuration
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  // Perform the search
  run(config);
}
