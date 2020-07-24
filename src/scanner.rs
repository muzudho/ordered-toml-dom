use crate::lexical_parser::LineParser;
use crate::syntax_parser::LineSyntaxScanner;
use casual_logger::Log;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LineScanner {}
impl LineScanner {
    pub fn from_file(path: &str) {
        Log::info(&format!("Read=|{}|", path));
        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!(Log::fatal(&format!("{}", why))),
                    };
                    Log::info(&format!("from_file/line=|{}|", line));
                    let mut token_line = LineParser::default();
                    token_line.parse_line(&line);
                    Log::info(&format!("from_file/line_tokens=|{:?}|", token_line));
                    let mut syntax_scanner = LineSyntaxScanner::default();
                    syntax_scanner.scan_line(&token_line.token_line);
                    Log::info(&format!("Finish| {:?}", syntax_scanner))
                }
            }
            Err(why) => panic!("{}", why),
        }
    }
}
