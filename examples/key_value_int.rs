//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_int`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-key-value-int");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/key-value-int.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

    // Test.
    let key = "int_max";
    if let Some(literal_string) = doc.get_literal_string_by_key(key) {
        if literal_string.value != "2147483647" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", literal_string.value)),
            );
        }
    } else {
        has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }

    // Test.
    let key = "int_min";
    if let Some(literal_string) = doc.get_literal_string_by_key(key) {
        if literal_string.value != "-2147483647" {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", literal_string.value)),
            );
        }
    } else {
        has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }

    // Error handling.
    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}
