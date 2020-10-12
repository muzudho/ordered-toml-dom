//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-key-value");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/key-value.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

    // Test.
    let key = "int_1";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        has_error = true;
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    // Test.
    let key = "float_1";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.2.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        has_error = true;
        Log::error_t("Test.2.", Table::default().str(key, ""));
    }

    // Test.
    let key = "sqstr_1";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.3.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        has_error = true;
        Log::error_t("Test.3.", Table::default().str(key, ""));
    }

    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}
