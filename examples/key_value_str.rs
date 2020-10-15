//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_str`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::{model::layer310::Document, Toml};

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-key-value-str");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

    // Test.
    test_apple(&doc, &mut has_error);
    test_basic_strings_letter(&doc, &mut has_error);
    test_multiline_basic_strings_letter(&doc, &mut has_error);
    test_literal_strings_letter(&doc, &mut has_error);
    test_multiline_literal_strings_letter(&doc, &mut has_error);
    test_basic_strings_punctuation(&doc, &mut has_error);
    test_multiline_basic_strings_punctuation(&doc, &mut has_error);
    test_multiline_basic_strings_trim_start(&doc, &mut has_error);
    test_literal_strings_punctuation(&doc, &mut has_error);
    test_multiline_literal_strings_punctuation(&doc, &mut has_error);

    // Error handling.
    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}

fn test_apple(doc: &Document, has_error: &mut bool) {
    // Test.
    let key = "apple";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "pie" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_basic_strings_letter(doc: &Document, has_error: &mut bool) {
    let key = "basic_strings_letter";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "Hello, world!!" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_multiline_basic_strings_letter(doc: &Document, has_error: &mut bool) {
    let key = "multiline_basic_strings_letter";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value
            != "Hello,
    world!!"
        {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_literal_strings_letter(doc: &Document, has_error: &mut bool) {
    let key = "literal_strings_letter";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "Hello, world!!" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_multiline_literal_strings_letter(doc: &Document, has_error: &mut bool) {
    let key = "multiline_literal_strings_letter";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value
            != "Hello,
    world!!"
        {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_basic_strings_punctuation(doc: &Document, has_error: &mut bool) {
    // Test.
    let key = "basic_strings_punctuation";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "., ={}[]'\"\\!?" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_multiline_basic_strings_punctuation(doc: &Document, has_error: &mut bool) {
    let key = "multiline_basic_strings_punctuation";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value
            != "., ={}[]\"'\"\"\\
!?"
        {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_multiline_basic_strings_trim_start(doc: &Document, has_error: &mut bool) {
    let key = "multiline_basic_strings_trim_start";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "The quick brown fox jumps over the lazy dog." {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_literal_strings_punctuation(doc: &Document, has_error: &mut bool) {
    let key = "literal_strings_punctuation";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value != "., ={}[]\"\\!?" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
fn test_multiline_literal_strings_punctuation(doc: &Document, has_error: &mut bool) {
    let key = "multiline_literal_strings_punctuation";
    if let Some(right_value) = doc.get_str_by_key(key) {
        if right_value
            != "., ={}[]'\"\\
    !?"
        {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", right_value)),
            );
        }
    } else {
        *has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }
}
