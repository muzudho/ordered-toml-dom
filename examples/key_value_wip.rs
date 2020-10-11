//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

use casual_logger::{Log, Table};
use tomboy_toml_dom::{
    model::{layer220::RightValue, layer230::DocumentElement},
    Toml,
};

fn main() {
    println!("Start.");
    Log::set_file_name("key-value");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/key-value.toml");
    Log::info_t(
        "Product.",
        Table::default()
            .uint("DocumentElementCount", doc.elements.len() as u128)
            .str("OutputDocument", &format!("{:?}", doc)),
    );
    for elem in doc.elements {
        match elem {
            DocumentElement::HeaderOfArrayOfTable(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("HeaderOfArrayOfTable", &format!("{:?}", m)),
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
            DocumentElement::HeaderOfTable(m) => {
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
