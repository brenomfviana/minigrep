//! This file is part of minigrep.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! Minigrep is a free software; you can redistribute it and/or modify it under
//! the terms of the MIT License.

use std::env;
use std::error::Error;
use std::fs;

/// Search configuration values.
pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  /// Gets the arguments, validates them and returns the search configuration.
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    // Skip program call
    args.next();
    // Get query string from arguments
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("you did not enter a string."),
    };
    // Get query filename from arguments
    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("you did not enter the filename"),
    };
    // Check if the case insensitive variable is active
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    // Check if there are still arguments
    if let Some(_) = args.next() {
      println!("  WARNING: You have entered more arguments than needed.");
    }
    // Return the search configuration
    Ok(Config{ query, filename, case_sensitive })
  }
}

/// Searches for the query string in the file content and returns list of the
/// found lines.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines().filter(|line| line.contains(query)).collect()
}

/// Converts all characters to lowercase, searches for the query string in the
/// file content and returns list of the found lines.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)
  -> Vec<&'a str> {
    // Convert query string to lowercase
    let query = query.to_lowercase();
    // Init list of found lines with the query string
    let mut results = Vec::new();
    for line in contents.lines() {
      // Look for the lines that contains the query string
      if line.to_lowercase().contains(&query) {
        results.push(line);
      }
    }
    results
}

/// Perform file reading and applies the query.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // Read the file
  let contents = fs::read_to_string(config.filename)?;
  // Search for the query string in the file
  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  // Print the found lines
  for line in results {
    println!("{}", line);
  }
  // Return Ok
  Ok(())
}

/// Test module.
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "Rust:\n\
      safe, fast, productive.\n\
      Pick three.\n\
      Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
