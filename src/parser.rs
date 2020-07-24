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
        println!("Read=|{}|", path);
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
                    let _line = LineParser::from_line(&line);
                }
            }
            Err(_why) => {
                // TODO error.
            }
        }
    }
}

/// WIP.
/// Line parser.
/// 行パーサー。
pub struct LineParser {
    key: String,
    right_value: String,
}
impl LineParser {
    pub fn from_line(line: &str) -> Self {
        println!("{}", line);
        enum MachineState {
            Start,
            Key,
            RightValue,
        }

        let mut state = MachineState::Start;
        let mut key = String::new();
        let mut right_value = String::new();

        let ch_vec: Vec<char> = line.chars().collect();
        for ch in ch_vec {
            match state {
                MachineState::Start => {
                    // 最初に出てくる文字まで飛ばします。
                    // Whitespace means tab (0x09 '\t') or space (0x20 ' ').
                    match ch {
                        '\t' | ' ' => println!("[WhiteSpace]"),
                        _ => {
                            println!("[{}]", ch);
                            key.push(ch);
                            state = MachineState::Key;
                        }
                    }
                }
                MachineState::Key => {
                    // イコールが出てくるまで読み取ります。
                    // スペースもキーに含めます。
                    match ch {
                        '=' => {
                            println!("key=|{}|", key);
                            state = MachineState::RightValue;
                        }
                        _ => {
                            key.push(ch);
                        }
                    }
                }
                MachineState::RightValue => {
                    right_value.push(ch);
                }
            }
        }
        println!("right_value=|{}|", right_value);

        /*
        if let Ok(re_toml_key) = RE_TOML_KEY.lock() {
            if re_toml_key.is_match(key) {
                // Ok.
                return key.to_string();
            }
        }
        */

        LineParser {
            key: key,
            right_value: right_value,
        }
    }
}

enum RightValueMachineState {
    Start,
    InlineTableParser,
}

/// Right value parser.
/// 右値パーサー
pub struct RightValueParser {
    state: RightValueMachineState,
}
impl Default for RightValueParser {
    fn default() -> Self {
        RightValueParser {
            state: RightValueMachineState::Start,
        }
    }
}
impl RightValueParser {
    pub fn parse(&mut self, ch: char) {
        println!("RightValueParser|ch=|{}|", ch);
        match self.state {
            RightValueMachineState::Start => {
                // 最初に出てくる文字まで飛ばします。
                match ch {
                    '\t' | ' ' => println!("[WhiteSpace]"),
                    '{' => {
                        println!("[{}]", ch);
                        self.state = RightValueMachineState::InlineTableParser;
                    }
                    _ => {
                        println!("[{}]", ch);
                        // key.push(ch);
                        // state = MachineState::Key;
                    }
                }
            }
            RightValueMachineState::InlineTableParser => {}
        }
    }
}

enum InlineTableMachineState {
    Start,
}
/// Inline table parser.
/// インライン・テーブル・パーサー
pub struct InlineTableParser {
    state: InlineTableMachineState,
}
impl Default for InlineTableParser {
    fn default() -> Self {
        InlineTableParser {
            state: InlineTableMachineState::Start,
        }
    }
}
impl InlineTableParser {
    pub fn parse(&mut self, ch: char) {
        println!("InlineTableParser|ch=|{}|", ch);

        match self.state {
            InlineTableMachineState::Start => {
                // 最初に出てくる文字まで飛ばします。
                match ch {
                    '\t' | ' ' => println!("[WhiteSpace]"),
                    _ => {
                        println!("[{}]", ch);
                        // key.push(ch);
                        // state = MachineState::Key;
                    }
                }
            }
        }
    }
}
