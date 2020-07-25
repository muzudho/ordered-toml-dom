use crate::lexical_parser::LineLexicalParser;
use crate::syntax::SyntaxParserResult;
use crate::syntax_parser::LineSyntaxScanner;
use casual_logger::{ArrayOfTable, Log, Table};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LineScanner {}
impl LineScanner {
    pub fn from_file(path: &str) {
        Log::info(&format!("Read=|{}|", path));
        let mut aot = ArrayOfTable::default().clone();
        // let mut dom =
        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    Log::info(&format!("from_file/line=|{}|", line));
                    let mut token_line = LineLexicalParser::default();
                    token_line.parse_line(&line);
                    Log::info(&format!("from_file/line_tokens=|{:?}|", token_line));
                    let mut line_syntax_scanner = LineSyntaxScanner::default();
                    match line_syntax_scanner.scan_line(&token_line.token_line) {
                        SyntaxParserResult::Ok(_) => {
                            let product = line_syntax_scanner.line_parser.product();
                            Log::info_t(
                                "Product",
                                Table::default()
                                    .str("parser", "LineScanner#from_file")
                                    .str("product", &format!("{:?}", product)),
                            );
                        } // Ignored it.
                        SyntaxParserResult::Err(table) => {
                            aot.table(
                                Table::default()
                                    .str("line", &format!("{}", line))
                                    .str("token_line", &format!("{:?}", token_line))
                                    .sub_t("error", &table)
                                    .sub_t("line_scanner", &line_syntax_scanner.log()),
                            );
                        }
                    }
                }
            }
            Err(why) => panic!("{}", why),
        }
        Log::info_t(
            "Finish of LineScanner#from_file().",
            Table::default().sub_aot("file", &aot),
        )
    }
}
