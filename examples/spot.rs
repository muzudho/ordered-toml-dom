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

    assert_eq!(doc.get_string_by_key("z"), Some("abc\txyz".to_string()));
}
