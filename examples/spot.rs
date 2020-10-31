//! Spot test.
//! 単発テスト。
//!
//! `cargo run --example spot`

extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/spot.toml");
    println!("display=|{}|", doc);
    println!("debug=|{:?}|", doc);

    assert_eq!(doc.get_i128_by_key_v2("i32_max"), Ok(Some(2147483647)));
}
