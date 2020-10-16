//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_str`

extern crate tomboy_toml_dom;

use tomboy_toml_dom::Toml;

fn main() {
    // Read a Toml file.
    let doc = Toml::from_file("./resource/example.toml");

    // Test.
    assert_eq!(doc.get_str_by_key("apple"), Some("pie"));

    assert_eq!(
        doc.get_str_by_key("basic_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
        )
    );
    assert_eq!(
        doc.get_str_by_key("literal_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
        )
    );
    assert_eq!(
        doc.get_str_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_punctuation"),
        Some(
            "., ={}[]\"'\"\"\\
!?"
        )
    );
    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_trim_start"),
        Some("The quick brown fox jumps over the lazy dog.")
    );
    assert_eq!(
        doc.get_str_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
        )
    );
}
