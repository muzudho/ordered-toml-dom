//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example table`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-table");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Read a Toml file.
    let doc = Toml::from_file("./resource/table.toml");
    Log::info_toml_document(&doc);

    let key = "food";
    if let Some(elem) = doc.get_key_value_by_key(key) {
        Log::info_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    Log::flush();
}
