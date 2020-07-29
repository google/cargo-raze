extern crate libloading;
extern crate regex;
extern crate specs;

use regex::Match;

fn main() {
  println!("hello world");

  // Make sure libloading is not optimized out
  let _lib = libloading::Library::new("/path/to/liblibrary.so");
}
