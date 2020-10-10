//! Test.
//! テスト。
//!
//! `cargo run --example main`

use casual_logger::{Log, Table};
use tomboy_toml_dom::{
    model::{layer220::RightValue, layer230::DocumentElement},
    Toml,
};

fn main() {
    println!("Start.");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/example.type.toml");
    Log::info_t(
        "Count document elements.",
        Table::default().uint("DocumentElementsCount", doc.elements.len() as u128),
    );
    for elem in doc.elements {
        match elem {
            DocumentElement::ArrayOfTable(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("ArrayOfTable", &format!("{:?}", m)),
                );
            }
            DocumentElement::Comment(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Comment", &format!("{:?}", m)),
                );
            }
            DocumentElement::EmptyLine => {
                Log::info_t("Scan a Broad-line.", Table::default().str("EmptyLine", ""));
            }
            DocumentElement::KeyValue(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
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
            DocumentElement::Table(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Table", &format!("{:?}", m)),
                );
            }
        }
    }
    Log::flush();
    println!("Finished.");
}
