//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use chrono::prelude::{DateTime, Utc};
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let doc = Toml::from_file("./resource/example.toml");

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_i128_by_key("age"), Some(40));
    assert_eq!(doc.get_i128_by_key("int_max"), Some(2147483647));
    assert_eq!(doc.get_i128_by_key("int_min"), Some(-2147483648));
    assert_eq!(doc.get_f64_by_key("weight"), Some(93.5));

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_str_by_key("apple"), Some("pie"));

    assert_eq!(
        doc.get_str_by_key("basic_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(doc.get_str_by_key("basic_string_empty"), Some(""));
    assert_eq!(
        doc.get_str_by_key("basic_string_escape_backslash"),
        Some("\\")
    );
    assert_eq!(
        doc.get_str_by_key("basic_string_escape_double_quotation"),
        Some("\"")
    );
    assert_eq!(
        doc.get_str_by_key("basic_string_punctuation"),
        Some("., ={}[]'\"\\!?")
    );
    // TODO assert_eq!(doc.get_str_by_key("basic_string_tab"), Some("a\tb"));

    assert_eq!(
        doc.get_str_by_key("multiline_basic_string_letter"),
        Some(
            "Hello,
world!!"
        )
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
        doc.get_str_by_key("multiline_basic_string_escape_double_quotation"),
        Some(
            "
\\
"
        )
    );
    /* TODO
        assert_eq!(
            doc.get_str_by_key("multiline_basic_string_tab"),
            Some(
                "
    a\tb
    "
            )
        );
        */

    assert_eq!(doc.get_str_by_key("literal_string_empty"), Some(""));
    assert_eq!(
        doc.get_str_by_key("literal_string_letter"),
        Some("Hello, world!!")
    );
    assert_eq!(
        doc.get_str_by_key("literal_string_punctuation"),
        Some("., ={}[]\"\\!?")
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_letter"),
        Some(
            "Hello,
world!!"
        )
    );
    assert_eq!(
        doc.get_str_by_key("multiline_literal_string_punctuation"),
        Some(
            "., ={}[]'\"\\
!?"
        )
    );

    // Read a boolean.
    // 論理値読取。
    assert_eq!(doc.get_bool_by_key("adult"), Some(true));
    assert_eq!(doc.get_bool_by_key("student"), Some(false));

    // DateTime.
    // 日付と時刻。
    assert_eq!(
        doc.get_datetime_utc_by_key("dob"),
        Some(
            "1979-05-27T07:32:00-08:00"
                .parse::<DateTime<Utc>>()
                .unwrap()
        )
    );
}
