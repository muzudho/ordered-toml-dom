//! WIP.
//! Test config file.
//! 設定ファイルのテスト。
//!
//! `cargo run --example main`

use casual_logger::{Log, Table};
use toml_menu::{
    model::{Element, Value},
    Toml,
};

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
            Element::Comment(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("Comment", &format!("{:?}", m)),
                );
            }
            Element::KeyValue(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("KeyValue", &format!("{:?}", m)),
                );
                match *m.value {
                    Value::Array(m) => Log::info(&format!("{:?}", m)),
                    Value::InlineTable(m) => Log::info(&format!("{:?}", m)),
                    Value::KeyValue(m) => Log::info(&format!("{:?}", m)),
                    Value::LiteralString(m) => Log::info(&format!("{:?}", m)),
                    Value::SingleQuotedString(m) => Log::info(&format!("{:?}", m)),
                }
            }
        }
    }
    Log::flush();
    println!("Finished.");
}
