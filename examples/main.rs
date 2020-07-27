//! Test.
//! テスト。
//!
//! `cargo run --example main`

use casual_logger::{Log, Table};
use toml_menu::{
    model::{BroadLine, RightValue},
    Toml,
};

fn main() {
    println!("Start.");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/example.type.toml");
    Log::info_t(
        "Count elements.",
        Table::default().uint("BroadLineCount", doc.broad_lines.len() as u128),
    );
    for elem in doc.broad_lines {
        match elem {
            BroadLine::Comment(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("Comment", &format!("{:?}", m)),
                );
            }
            BroadLine::KeyValue(m) => {
                Log::info_t(
                    "Scan a element.",
                    Table::default().str("KeyValue", &format!("{:?}", m)),
                );
                match *m.value {
                    RightValue::Array(m) => Log::info(&format!("{:?}", m)),
                    RightValue::DoubleQuotedString(m) => Log::info(&format!("{:?}", m)),
                    RightValue::InlineTable(m) => Log::info(&format!("{:?}", m)),
                    RightValue::LiteralString(m) => Log::info(&format!("{:?}", m)),
                    RightValue::SingleQuotedString(m) => Log::info(&format!("{:?}", m)),
                }
            }
        }
    }
    Log::flush();
    println!("Finished.");
}
