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
        let mut aot = ArrayOfTable::default().clone();
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
                    let mut line_syntax_scanner = DocumentParser::default();
                    match line_syntax_scanner.scan_line(&lexical_p.product(), &mut doc) {
                        PResult::End => {} // Ignored it.
                        PResult::Err(table) => {
                            aot.table(
                                Toml::log_table("code.65.")
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
                                    .sub_t("error", &table)
                                    .sub_t(
                                        "line_scanner",
                                        &line_syntax_scanner.log_table("lib.rs.77."),
                                    ),
                            );
                        }
                        PResult::Ongoing => {
                            aot.table(
                                Toml::log_table("code.85")
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
                                    .sub_t(
                                        "line_scanner",
                                        &line_syntax_scanner.log_table("code.96."),
                                    ),
                            );
                        }
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }
        Log::info_t(
            "Product.",
            Toml::log_table("code.109.").str("product_dom", &format!("{:?}", doc)),
        );
        Log::info_t(
            "Finish of Toml#from_file().",
            Toml::log_table("code.113.").sub_aot("file", &aot),
        );

        doc
    }
    pub fn log_table(place_of_occurrence: &str) -> Table {
        let t = Table::default()
            .str("Parse", "lib.rs/Toml#from_file")
            .str("place_of_occurrence", place_of_occurrence)
            .clone();
        t
    }
}
