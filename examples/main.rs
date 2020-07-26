//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use casual_logger::{Log, Table};
use toml_menu::Toml;

fn main() {
    println!("Start.");
    Log::remove_old_logs();
    let doc = Toml::from_file("./example.type.toml");
    Log::info_t(
        "Count elements.",
        Table::default().uint("count", doc.elements.len() as u128),
    );
    for elem in doc.elements {
        Log::info_t(
            "Scan a element.",
            Table::default().str("elem", &format!("{:?}", elem)),
        );
    }
    Log::flush();
    println!("Finished.");
}
