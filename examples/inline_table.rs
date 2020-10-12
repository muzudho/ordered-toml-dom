//! Test.
//! テスト。
//!
//! `cargo run --example mix_array`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("test-inline-table");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();
    Log::println("Start.");

    // Read a Toml file.
    let doc = Toml::from_file("./resource/inline-table.toml");
    Log::info_toml_document(&doc);

    // Test.
    let key = "inline_table_3";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    Log::flush();
    Log::println("Finished.");
}
