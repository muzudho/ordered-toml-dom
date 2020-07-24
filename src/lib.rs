//! `cargo run --example main`

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod parser;
pub mod tokenizer;

use regex::Regex;
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    pub static ref RE_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"[A-Za-z0-9_-]$").unwrap());
}
