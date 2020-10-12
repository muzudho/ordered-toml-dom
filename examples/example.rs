//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    // Read a number.
    // 数値読取。
    if let Some(age) = doc.get_int128_by_key("age") {
        println!("age = {}", age);
        // age = 40
    }

    // Read a string.
    // 文字列読取。
    if let Some(apple) = doc.get_str_by_key("apple") {
        println!("apple = {}", apple);
        // apple = pie
    }
}
