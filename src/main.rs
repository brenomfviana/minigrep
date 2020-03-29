//! This file is part of minigrep.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! Minigrep is a free software; you can redistribute it and/or modify it under
//! the terms of the MIT License.

use std::env;
use std::process;
use minigrep::Config;

fn main() {
  println!("Minigrep");
  // Get arguments
  let args: Vec<String> = env::args().collect();
  // Get the search configuration
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
  });
  // Perform the search
  if let Err(e) = minigrep::run(config) {
    println!("Application error: {}.", e);
    process::exit(1);
  }
}
