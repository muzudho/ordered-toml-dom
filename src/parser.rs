//! WIP.
//! TOML parser.
//! TOMLパーサー。

// use crate::toml::auto_correct::RE_TOML_KEY;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// WIP.
/// TOML parser.
/// TOMLパーサー。
pub struct Document {}
impl Document {
    pub fn from_file(path: &str) {
        println!("WIP. {}", path);
        match File::open(path) {
            Ok(file) => {
                for line in BufReader::new(file).lines() {
                    let line = match line {
                        Ok(line) => line,
                        Err(_why) => {
                            // TODO error.
                            return;
                        }
                    };
                    let line = LineSeek::from_line(&line);
                }
            }
            Err(_why) => {
                // TODO error.
            }
        }
    }
}

/// WIP.
/// Line seeker.
/// 行シーカー。
pub struct LineSeek {}
impl LineSeek {
    pub fn from_line(line: &str) -> Self {
        println!("{}", line);
        // 最初に出てくる文字まで飛ばします。
        // Whitespace means tab (0x09 '\t') or space (0x20 ' ').
        let ch_vec: Vec<char> = line.chars().collect();
        let mut count = 0;
        for ch in ch_vec {
            match ch {
                '\t' | ' ' => println!("[WhiteSpace]"),
                _ => println!("[{}]", ch),
            }
            count += 1;
        }

        /*
        if let Ok(re_toml_key) = RE_TOML_KEY.lock() {
            if re_toml_key.is_match(key) {
                // Ok.
                return key.to_string();
            }
        }
        */

        LineSeek {}
    }
}
