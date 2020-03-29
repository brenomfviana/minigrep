//! This file is part of minigrep.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! Minigrep is a free software; you can redistribute it and/or modify it under
//! the terms of the MIT License.

use std::error::Error;
use std::fs;

/// Search configuration values.
pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  /// Gets the arguments, validates them and returns the search configuration.
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    // Check if the arguments are valid
    match args.len() {
      3 => { /* Right number of arguments. */ },
      0 => return Err("how did you do it?"),
      1 => return Err("you did not enter any arguments."),
      2 => return Err("you did not enter the filename."),
      _ => println!("  WARNING: You have entered more arguments than needed."),
    }
    // Return the search configuration
    let (query, filename) = (args[1].clone(), args[2].clone());
    Ok(Config{ query, filename })
  }
}

/// Perform file reading and applies the query.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // Read the file
  let contents = fs::read_to_string(config.filename)?;
  // Search for the query string in the file and print the found lines
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }
  // Return Ok
  Ok(())
}

/// Searches for the query string in the file content and returns list of the
/// found lines.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // Init list of found lines with the query string
  let mut results = Vec::new();
  // Look for the lines that contains the query string
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }
  // Return result
  results
}

/// Test module.
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
}
