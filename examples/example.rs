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
    test_age(&doc);
    test_weight(&doc);

    // WIP. Read a string.
    // 作業中。 文字列読取。
    test_apple(&doc);
    test_basic_strings_empty(&doc);
    test_basic_strings_escape_backslash(&doc);
    test_basic_strings_escape_double_quotation(&doc);
    test_basic_strings_punctuation(&doc);
    test_multiline_basic_strings_letter(&doc);
    test_multiline_basic_strings_punctuation(&doc);
    test_multiline_basic_strings_trim_start(&doc);

    // Read a boolean.
    // 論理値読取。
    test_boolean_true(&doc);
    test_boolean_false(&doc);
}

fn test_age(doc: &Document) {
    if let Some(age) = doc.get_i128_by_key("age") {
        println!("age = {}", age);
        // age = 40
    }
}
fn test_weight(doc: &Document) {
    if let Some(age) = doc.get_f64_by_key("weight") {
        println!("weight = {}", age);
        // weight = 93.5
    }
}
fn test_apple(doc: &Document) {
    // "pie"
    if let Some(apple) = doc.get_str_by_key("apple") {
        println!("apple = {}", apple);
        // apple = pie
    }
}
fn test_basic_strings_empty(doc: &Document) {
    // ""
    if let Some(basic_strings_empty) = doc.get_str_by_key("basic_strings_empty") {
        println!("basic_strings_empty = {}", basic_strings_empty);
        // basic_strings_empty =
    }
}
fn test_basic_strings_escape_backslash(doc: &Document) {
    // "\\"
    if let Some(basic_strings_escape_backslash) =
        doc.get_str_by_key("basic_strings_escape_backslash")
    {
        println!(
            "basic_strings_escape_backslash = {}",
            basic_strings_escape_backslash
        );
        // basic_strings_escape_backslash = \
    }
}
fn test_basic_strings_escape_double_quotation(doc: &Document) {
    // "\""
    if let Some(basic_strings_escape_double_quotation) =
        doc.get_str_by_key("basic_strings_escape_double_quotation")
    {
        println!(
            "basic_strings_escape_double_quotation = {}",
            basic_strings_escape_double_quotation
        );
        // basic_strings_escape_double_quotation = \
    }
}
fn test_basic_strings_punctuation(doc: &Document) {
    // "., ={}[]'\"\\!?"
    if let Some(basic_strings_punctuation) = doc.get_str_by_key("basic_strings_punctuation") {
        println!("basic_strings_punctuation = {}", basic_strings_punctuation);
        // basic_strings_punctuation = ., ={}[]'"\!?
    }
}
fn test_multiline_basic_strings_letter(doc: &Document) {
    // """Hello,
    // world!!"""
    if let Some(multiline_basic_strings_letter) =
        doc.get_str_by_key("multiline_basic_strings_letter")
    {
        println!(
            "multiline_basic_strings_letter = {}",
            multiline_basic_strings_letter
        );
        // multiline_basic_strings_letter = Hello,
        // world!!
    }
}
fn test_multiline_basic_strings_punctuation(doc: &Document) {
    // """., ={}[]"'""\\
    // !?"""
    if let Some(multiline_basic_strings_punctuation) =
        doc.get_str_by_key("multiline_basic_strings_punctuation")
    {
        println!(
            "multiline_basic_strings_punctuation = {}",
            multiline_basic_strings_punctuation
        );
        // multiline_basic_strings_punctuation = ., ={}[]"'""\
        // !?
    }
}
fn test_multiline_basic_strings_trim_start(doc: &Document) {
    // """\
    //   The quick brown \
    //   fox jumps over \
    //   the lazy dog.\
    //   """
    if let Some(multiline_basic_strings_trim_start) =
        doc.get_str_by_key("multiline_basic_strings_trim_start")
    {
        println!(
            "multiline_basic_strings_trim_start = {}",
            multiline_basic_strings_trim_start
        );
        // multiline_basic_strings_trim_start =
    }
}
fn test_boolean_true(doc: &Document) {
    if let Some(adult) = doc.get_bool_by_key("adult") {
        println!("adult = {}", adult);
        // adult = true
    }
}
fn test_boolean_false(doc: &Document) {
    if let Some(student) = doc.get_bool_by_key("student") {
        println!("student = {}", student);
        // student = false
    }
}
