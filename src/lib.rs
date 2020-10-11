//! TOML parser.  
//! トムル解析器。  
//!
//! `cargo run --example main`

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod model;
mod parser;

use crate::model::layer310::Document;
use crate::parser::{
    phase100::lexical_parser::LexicalParser,
    phase200::{layer210::PResult, layer240::document::DocumentParser},
};
use casual_logger::{ArrayOfTable, Log, Table};
use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    /// ドットは含みません。
    pub static ref RE_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"[A-Za-z0-9_-]$").unwrap());
}

/// TOML.  
/// トムル。  
pub struct Toml {}
impl Toml {
    /// Line scan.
    /// 行走査。
    pub fn from_file(path: &str) -> Document {
        Log::info(&format!("Read=|{}|", path));
        let mut info_aot = ArrayOfTable::default().clone();
        let mut error_aot = ArrayOfTable::default().clone();
        let mut doc = Document::default();
        match File::open(path) {
            Ok(file) => {
                for (i, line) in BufReader::new(file).lines().enumerate() {
                    let row_number = i + 1;
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    Log::trace(&format!("from_file/line=|{}|", line));
                    let mut lexical_p = LexicalParser::new(row_number);
                    lexical_p.parse_line(&line);
                    /*
                    Log::trace_t(
                        "Toml::from_file/line_tokens",
                        Toml::err_table()
                            .str("line", &line)
                            .str("token_line", &format!("=|{:?}|", lexical_p)),
                    );
                    */
                    let mut document_p = DocumentParser::default();
                    match document_p.scan_line(&lexical_p.product(), &mut doc) {
                        PResult::End => {} // Ignored it.
                        PResult::Err(mut table) => {
                            error_aot.table(
                                table.sub_t(
                                    "snapshot",
                                    Toml::log_snapshot()
                                        .str("via", "lib.rs.65.")
                                        .int(
                                            "row_number",
                                            if let Ok(n) = row_number.try_into() {
                                                n
                                            } else {
                                                -1
                                            },
                                        )
                                        .str("line", &format!("{}", line))
                                        .str("token_line", &format!("{:?}", lexical_p))
                                        .sub_t("line_scanner", &document_p.log_snapshot()),
                                ),
                            );
                        }
                        PResult::Ongoing => {
                            info_aot.table(
                                Toml::log_snapshot()
                                    .str("place_of_occurrence", "lib.rs.85.")
                                    .int(
                                        "row_number",
                                        if let Ok(n) = row_number.try_into() {
                                            n
                                        } else {
                                            -1
                                        },
                                    )
                                    .str("line", &format!("{}", line))
                                    .str("token_line", &format!("{:?}", lexical_p))
                                    .sub_t("line_scanner", &document_p.log_snapshot()),
                            );
                        }
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }
        Log::info_t(
            "Product.",
            Toml::log_snapshot()
                .str("place_of_occurrence", "lib.rs.109.")
                .str("product_dom", &format!("{:?}", doc)),
        );
        Log::info_t(
            "Finish of Toml#from_file().",
            Toml::log_snapshot()
                .str("place_of_occurrence", "lib.rs.113.")
                .sub_aot("file", &info_aot),
        );
        Log::error_t(
            "Finish of Toml#from_file() error.",
            Toml::log_snapshot()
                .str("place_of_occurrence", "lib.rs.121.")
                .sub_aot("file", &error_aot),
        );

        doc
    }
    pub fn log_snapshot() -> Table {
        let t = Table::default()
            .str("Parse", "lib.rs/Toml#from_file")
            .clone();
        t
    }
}
