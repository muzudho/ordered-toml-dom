use crate::lexical_parser::LineParser;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct LineScanner {}
impl LineScanner {
    pub fn from_file(path: &str) {
        println!("Read=|{}|", path);
        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    let line = match line {
                        Ok(line) => line,
                        Err(why) => panic!("{}", why),
                    };
                    println!("from_file/line=|{}|", line);
                    let mut line_tokens = LineParser::default();
                    line_tokens.parse_line(&line);
                    println!("from_file/line_tokens=|{:?}|", line_tokens);
                }
            }
            Err(why) => panic!("{}", why),
        }
    }
}
