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
use crate::parser::syntax::{
    layer110::lexical_parser::LexicalParser, layer210::PResult,
    layer310::syntax_scanner::SyntaxScanner,
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
                    let mut line_syntax_scanner = SyntaxScanner::default();
                    match line_syntax_scanner.scan_line(&lexical_p.product(), &mut doc) {
                        PResult::End => {} // Ignored it.
                        PResult::Err(table) => {
                            aot.table(
                                Toml::err_table()
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
                                    .sub_t("line_scanner", &line_syntax_scanner.err_table()),
                            );
                        }
                        PResult::Ongoing => {
                            aot.table(
                                Toml::err_table()
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
                                    .sub_t("line_scanner", &line_syntax_scanner.err_table()),
                            );
                        }
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }
        Log::info_t(
            "Product.",
            Toml::err_table().str("product_dom", &format!("{:?}", doc)),
        );
        Log::info_t(
            "Finish of Toml#from_file().",
            Toml::err_table().sub_aot("file", &aot),
        );

        doc
    }
    pub fn err_table() -> Table {
        let t = Table::default()
            .str("Parse", "lib.rs/Toml#from_file")
            .clone();
        t
    }
}
