//! `cargo run --example main`

#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod lexical_parser;
pub mod model;
mod syntax;
pub mod syntax_scanner;
mod token;

use crate::lexical_parser::LexicalParser;
use crate::model::Document;
use crate::syntax::SyntaxParserResult;
use crate::syntax_scanner::LineSyntaxScanner;
use casual_logger::{ArrayOfTable, Log, Table};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

lazy_static! {
    /// Without dot.
    pub static ref RE_KEY: Mutex<Regex> = Mutex::new(Regex::new(r"[A-Za-z0-9_-]$").unwrap());
}

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
                for line in BufReader::new(file).lines() {
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    Log::info(&format!("from_file/line=|{}|", line));
                    let mut token_line = LexicalParser::default();
                    token_line.parse_line(&line);
                    Log::info_t(
                        "from_file/line_tokens",
                        Table::default()
                            .str("line", &line)
                            .str("token_line", &format!("=|{:?}|", token_line)),
                    );
                    let mut line_syntax_scanner = LineSyntaxScanner::default();
                    match line_syntax_scanner.scan_line(&token_line.product(), &mut doc) {
                        SyntaxParserResult::End => {} // Ignored it.
                        SyntaxParserResult::Err(table) => {
                            aot.table(
                                Table::default()
                                    .str("line", &format!("{}", line))
                                    .str("token_line", &format!("{:?}", token_line))
                                    .sub_t("error", &table)
                                    .sub_t("line_scanner", &line_syntax_scanner.err_table()),
                            );
                        }
                        SyntaxParserResult::Ongoing => {
                            aot.table(
                                Table::default()
                                    .str("line", &format!("{}", line))
                                    .str("token_line", &format!("{:?}", token_line))
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
            Table::default()
                .str("parser", "scanners.rs/Toml#from_file")
                .str("product_dom", &format!("{:?}", doc)),
        );
        Log::info_t(
            "Finish of Toml#from_file().",
            Table::default().sub_aot("file", &aot),
        );

        doc
    }
}
