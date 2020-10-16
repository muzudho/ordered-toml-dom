//! An exemplary program.
//! 模範的なプログラム。
//!
//! `cargo run --example example`

extern crate tomboy_toml_dom;

use tomboy_toml_dom::model::layer310::Document;
use tomboy_toml_dom::Toml;

fn main() {
    // Read a toml.
    // Toml読取。
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    // Read a number.
    // 数値読取。
    assert_eq!(doc.get_i128_by_key("age"), Some(40));
    assert_eq!(doc.get_f64_by_key("weight"), Some(93.5));

    // WIP. Read a string.
    // 作業中。 文字列読取。
    assert_eq!(doc.get_str_by_key("apple"), Some("pie"));

    assert_eq!(doc.get_str_by_key("basic_string_empty"), Some(""));
    assert_eq!(
        doc.get_str_by_key("basic_string_escape_backslash"),
        Some("\\")
    );

    test_basic_string_escape_double_quotation(&doc);
    test_basic_string_punctuation(&doc);

    test_multiline_basic_string_letter(&doc);
    test_multiline_basic_string_punctuation(&doc);
    test_multiline_basic_string_trim_start(&doc);

    test_literal_string_empty(&doc);
    test_literal_string_letter(&doc);

    test_multiline_literal_string_letter(&doc);

    // Read a boolean.
    // 論理値読取。
    test_boolean_true(&doc);
    test_boolean_false(&doc);
}

fn test_basic_string_escape_double_quotation(doc: &Document) {
    // "\""
    if let Some(actual) = doc.get_str_by_key("basic_string_escape_double_quotation") {
        println!("basic_string_escape_double_quotation = {}", actual);
        // basic_string_escape_double_quotation = \
    }
}
fn test_basic_string_punctuation(doc: &Document) {
    // "., ={}[]'\"\\!?"
    if let Some(actual) = doc.get_str_by_key("basic_string_punctuation") {
        println!("basic_string_punctuation = {}", actual);
        // basic_string_punctuation = ., ={}[]'"\!?
    }
}
fn test_multiline_basic_string_letter(doc: &Document) {
    // """Hello,
    // world!!"""
    if let Some(actual) = doc.get_str_by_key("multiline_basic_string_letter") {
        println!("multiline_basic_string_letter = {}", actual);
        // multiline_basic_string_letter = Hello,
        // world!!
    }
}
fn test_multiline_basic_string_punctuation(doc: &Document) {
    // """., ={}[]"'""\\
    // !?"""
    if let Some(actual) = doc.get_str_by_key("multiline_basic_string_punctuation") {
        println!("multiline_basic_string_punctuation = {}", actual);
        // multiline_basic_string_punctuation = ., ={}[]"'""\
        // !?
    }
}
fn test_multiline_basic_string_trim_start(doc: &Document) {
    // """\
    //   The quick brown \
    //   fox jumps over \
    //   the lazy dog.\
    //   """
    if let Some(actual) = doc.get_str_by_key("multiline_basic_string_trim_start") {
        println!("multiline_basic_string_trim_start = {}", actual);
        // multiline_basic_string_trim_start =
    }
}
fn test_literal_string_empty(doc: &Document) {
    // ""
    if let Some(actual) = doc.get_str_by_key("literal_string_empty") {
        println!("literal_string_empty = {}", actual);
        // literal_string_empty =
    }
}
fn test_literal_string_letter(doc: &Document) {
    // ""
    if let Some(actual) = doc.get_str_by_key("literal_string_letter") {
        println!("literal_string_letter = {}", actual);
        // literal_string_letter =
    }
}
fn test_multiline_literal_string_letter(doc: &Document) {
    // '''Hello,
    // world!!'''
    if let Some(actual) = doc.get_str_by_key("multiline_literal_string_letter") {
        println!("multiline_literal_string_letter = {}", actual);
        // multiline_literal_string_letter = Hello,
        // world!!
    }
}
fn test_boolean_true(doc: &Document) {
    if let Some(actual) = doc.get_bool_by_key("adult") {
        println!("adult = {}", actual);
        // adult = true
    }
}
fn test_boolean_false(doc: &Document) {
    if let Some(actual) = doc.get_bool_by_key("student") {
        println!("student = {}", actual);
        // student = false
    }
}
