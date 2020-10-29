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

    // Read a array.
    // 配列読取。
    assert_eq!(
        doc.get_string_array_by_key("string_array"),
        Ok(Some(vec![
            "a".to_string(),
            "b".to_string(),
            "\"c\"".to_string()
        ]))
    );
}
