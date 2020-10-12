//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::model::layer220::RightValue;
use tomboy_toml_dom::model::layer230::DocumentElement::KeyValue;
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
    if let Some(elem) = doc.get_key_value_by_key(key) {
        if let KeyValue(key_value) = elem {
            if key_value.key != key {
                Log::error_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
            }
            if let RightValue::LiteralString(literal_string) = &*key_value.value {
                if literal_string.value != "2147483647" {
                    Log::error_t(
                        "Test.1.",
                        Table::default().str(key, &format!("{:?}", literal_string.value)),
                    );
                }
            }
        } else {
            Log::error_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
        }
    } else {
        has_error = true;
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}
