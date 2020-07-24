//! WIP.
//! TOML parser.
//! TOMLパーサー。

// use crate::toml::auto_correct::RE_TOML_KEY;
use casual_logger::{Log, Table};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// WIP.
/// TOML parser.
/// TOMLパーサー。
pub struct Document {}
impl Document {
    pub fn from_file(path: &str) {
        Log::info_t("from_file", Table::default().str("path", path));
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
        Log::info_t("from_line", Table::default().str("line", line));
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
                        '\t' | ' ' => Log::info("[WhiteSpace]"),
                        _ => {
                            Log::info_t("from_line", Table::default().char("ch", ch));
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
                            Log::info_t("from_line", Table::default().str("key", &key));
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
        Log::info_t(
            "from_line",
            Table::default().str("right_value", &right_value),
        );

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
        Log::info_t("RightValueParser#parse", Table::default().char("ch", ch));
        match self.state {
            RightValueMachineState::Start => {
                // 最初に出てくる文字まで飛ばします。
                match ch {
                    '\t' | ' ' => Log::info("[WhiteSpace]"),

                    '{' => {
                        self.state = RightValueMachineState::InlineTableParser;
                    }
                    _ => {
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
        Log::info_t("InlineTableParser#parse", Table::default().char("ch", ch));

        match self.state {
            InlineTableMachineState::Start => {
                // 最初に出てくる文字まで飛ばします。
                match ch {
                    '\t' | ' ' => Log::info("[WhiteSpace]"),
                    _ => {
                        // key.push(ch);
                        // state = MachineState::Key;
                    }
                }
            }
        }
    }
}
