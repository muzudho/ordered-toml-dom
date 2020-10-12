//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value_str`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-key-value-str");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/example.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

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
        has_error = true;
        Log::error_t(&format!("Test: {}", key), Table::default().str(key, ""));
    }

    // Error handling.
    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}
