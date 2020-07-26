//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use casual_logger::{Log, Table};
use toml_menu::{model::ElementM, Toml};

fn main() {
    println!("Start.");
    Log::remove_old_logs();
    let doc = Toml::from_file("./example.type.toml");
    Log::info_t(
        "Count elements.",
        Table::default().uint("count", doc.elements.len() as u128),
    );
    for elem in doc.elements {
        match elem {
            ElementM::Comment(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("Comment", &format!("{:?}", m)),
                );
            }
            ElementM::KeyValue(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("KeyValue", &format!("{:?}", m)),
                );
            }
        }
    }
    Log::flush();
    println!("Finished.");
}
