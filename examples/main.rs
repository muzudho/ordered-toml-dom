//! Test.
//! テスト。
//!
//! `cargo run --example main`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Log, Table};
use tomboy_toml_dom::{
    model::{layer220::RightValue, layer230::DocumentElement},
    Toml,
};

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-main");
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/example.type.toml";
    let doc = Toml::from_file(toml_file);
    Log::info_toml_document(toml_file, &doc);

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
}
