//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example table`

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
    Log::set_file_name("table");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/table.toml");
    Log::println_t(
        "Count document elements.",
        Table::default().uint("DocumentElementsCount", doc.elements.len() as u128),
    );
    let a_name = "food";
    let a_value = if let Some(elem) = doc.get_element_by_name(a_name) {
        format!("{:?}", elem)
    } else {
        format!("")
    };
    Log::println_t(
        &format!("Test {}.", a_name),
        Table::default().str(a_name, &a_value),
    );
    Log::flush();
    Log::println("Finished.");
}
