//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

pub trait LogExt {
    fn println(s: &str);
    fn println_t(s: &str, t: &mut Table);
}
impl LogExt for Log {
    /// Info level logging and add print to stdout.
    fn println(s: &str) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln(s);
    }

    /// Info level logging and add print to stdout.
    fn println_t(s: &str, t: &mut Table) {
        if Log::enabled(Level::Info) {
            println!("{}", s);
        }
        Log::infoln_t(s, t);
    }
}

fn main() {
    Log::println("Start.");
    Log::set_file_name("key-value-int");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/key-value-int.toml");
    Log::info_t(
        "Product.",
        Table::default()
            .uint("DocumentElementCount", doc.elements.len() as u128)
            .str("OutputDocument", &format!("{:?}", doc)),
    );

    // Test.
    let key = "int_1";
    let key_value = if let Some(elem) = doc.get_key_value_by_key(key) {
        format!("{:?}", elem)
    } else {
        format!("NotFound")
    };
    Log::println_t("Test.1.", Table::default().str(key, &key_value));

    Log::flush();
    Log::println("Finished.");
}
