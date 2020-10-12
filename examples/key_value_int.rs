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
    Log::println("Start.");
    Log::set_file_name("key-value-int");
    Log::remove_old_logs();

    // Read a Toml file.
    let doc = Toml::from_file("./resource/key-value-int.toml");
    Log::info_toml_document(&doc);

    // Test.
    let key = "int_1";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    Log::flush();
    Log::println("Finished.");
}
