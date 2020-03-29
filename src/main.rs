//! This file is part of minigrep.
//!
//! Copyright (c) 2020 by Breno Viana.
//!
//! Minigrep is a free software; you can redistribute it and/or modify it under
//! the terms of the MIT License.

use std::env;

/// Gets the arguments and validate them.
fn get_args() -> (String, String) {
  // Get arguments
  let args: Vec<String> = env::args().collect();
  // Check if the arguments are valid
  match args.len() {
    0 => panic!("ERROR: How do you did that?"),
    1 => panic!("ERROR: You did not enter any arguments."),
    2 => panic!("ERROR: You did not enter the filename."),
    _ => println!("WARNING: You entered more arguments than the necessary."),
  }
  // Return the tuple of arguments
  (args[1].clone(), args[2].clone())
}

fn main() {
  // Get query key and filename
  let (query, filename) = get_args();
  // Print arguments
  println!("Searching for {}", query);
  println!("In file {}", filename);
}
