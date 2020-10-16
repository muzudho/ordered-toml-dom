//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_number`

extern crate tomboy_toml_dom;

mod modules;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a Toml file.
    let doc = Toml::from_file("./resource/key-value-number.toml");

    // Test.
    assert_eq!(doc.get_i128_by_key("int_max"), Some(2147483647));
    assert_eq!(doc.get_i128_by_key("int_min"), Some(-2147483648));
}
