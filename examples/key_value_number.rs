//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_number`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-key-value-number");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/key-value-number.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

    // Test.
    let key = "int_max";
    if let Some(number) = doc.get_i128_by_key(key) {
        if number != 2147483647 {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", number)),
            );
        }
    } else {
        has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }

    // Test.
    let key = "int_min";
    if let Some(number) = doc.get_i128_by_key(key) {
        if number != -2147483647 {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", number)),
            );
        }
    } else {
        has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }

    // Test.
    let key = "float_1";
    if let Some(number) = doc.get_f64_by_key(key) {
        if number != 3.14 {
            Log::error_t(
                &format!("Test: {}", key),
                Table::default().str(key, &format!("{:?}", number)),
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
