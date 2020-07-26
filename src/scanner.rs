use crate::lexical_parser::LineLexicalParser;
use crate::object_model::document::DocumentM;
use crate::syntax::SyntaxParserResult;
use crate::syntax_scanner::LineSyntaxScanner;
use casual_logger::{ArrayOfTable, Log, Table};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LineScanner {}
impl LineScanner {
    pub fn from_file(path: &str) {
        Log::info(&format!("Read=|{}|", path));
        let mut aot = ArrayOfTable::default().clone();
        let mut dom = DocumentM::default();
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
                    Log::info_t(
                        "from_file/line_tokens",
                        Table::default()
                            .str("line", &line)
                            .str("token_line", &format!("=|{:?}|", token_line)),
                    );
                    let mut line_syntax_scanner = LineSyntaxScanner::default();
                    match line_syntax_scanner.scan_line(&token_line.product(), &mut dom) {
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
                .str("parser", "scanners.rs/LineScanner#from_file")
                .str("product_dom", &format!("{:?}", dom)),
        );
        Log::info_t(
            "Finish of LineScanner#from_file().",
            Table::default().sub_aot("file", &aot),
        )
    }
}
