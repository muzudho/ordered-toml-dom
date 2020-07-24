//! 単語単位に分けます。
use crate::RE_KEY;
use casual_logger::{Log, Table};
use std::fmt;

#[derive(Debug)]
enum LineMachineState {
    Key,
    /// Whitespace means tab (0x09 '\t') or space (0x20 ' ').
    WhiteSpace,
}

/// WIP.
/// Line parser.
/// 行パーサー。
pub struct LineParser {
    state: Option<LineMachineState>,
    pub token_line: TokenLine,
    buf_token_type: TokenType,
    buf: String,
}
impl Default for LineParser {
    fn default() -> Self {
        LineParser {
            state: None,
            token_line: TokenLine::default(),
            buf_token_type: TokenType::WhiteSpace,
            buf: String::new(),
        }
    }
}
impl LineParser {
    pub fn parse_line(&mut self, line: &str) {
        Log::info_t("parse_line", Table::default().str("line", line));

        let ch_vec: Vec<char> = line.chars().collect();
        for ch in ch_vec {
            Log::info_t("parse_line", Table::default().char("ch", ch));
            if let Some(state) = &self.state {
                Log::info_t(
                    "parse_line",
                    Table::default()
                        .str("state", &format!("{:?}", state))
                        .char("ch", ch),
                );
                match state {
                    LineMachineState::WhiteSpace => {
                        // 最初に出てくる文字まで飛ばします。
                        match ch {
                            '\t' | ' ' => {
                                self.buf.push(ch);
                                Log::info_t(
                                    "",
                                    Table::default()
                                        .str("state", &format!("{:?}", state))
                                        .char("ch", ch),
                                );
                            }
                            _ => {
                                self.flush();
                                self.initial(ch);
                            }
                        }
                    }
                    LineMachineState::Key => {
                        let matched = match RE_KEY.lock() {
                            Ok(re_key) => re_key.is_match(&ch.to_string()),
                            Err(why) => panic!(Log::fatal(&format!("{}", why))),
                        };
                        if matched {
                            // A key.
                            self.buf.push(ch);
                            Log::info_t(
                                "",
                                Table::default()
                                    .str("state", &format!("{:?}", state))
                                    .char("ch", ch),
                            );
                        } else {
                            self.flush();
                            self.initial(ch);
                        }
                    }
                }
            } else {
                Log::info_t("parse_line", Table::default().char("ch", ch));
                self.flush();
                self.initial(ch);
            }
            Log::info_t("End of parse_line", Table::default().char("ch", ch));
        }
        Log::info("End of line.");
        self.flush();
    }
    /// Flush.
    fn flush(&mut self) {
        if !self.buf.is_empty() {
            self.token_line
                .tokens
                .push(Token::new(&self.buf, self.buf_token_type));
            Log::info_t("Flush", Table::default().str("buf", &self.buf));
            self.buf.clear();
            self.state = None;
        }
    }
    /// 最初の文字
    fn initial(&mut self, ch: char) {
        self.buf.push(ch);
        Log::info_t("Begin of initial", Table::default().char("ch", ch));
        match ch {
            '\t' | ' ' => {
                self.buf_token_type = TokenType::WhiteSpace;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                self.state = Some(LineMachineState::WhiteSpace);
            }
            ',' => {
                self.buf_token_type = TokenType::Comma;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            '=' => {
                self.buf_token_type = TokenType::Equals;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            '{' => {
                self.buf_token_type = TokenType::LeftCurlyBracket;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            '}' => {
                self.buf_token_type = TokenType::RightCurlyBracket;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            '#' => {
                self.buf_token_type = TokenType::Sharp;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            '\'' => {
                self.buf_token_type = TokenType::SingleQuotation;
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
            }
            _ => {
                let matched = match RE_KEY.lock() {
                    Ok(re_key) => re_key.is_match(&ch.to_string()),
                    Err(why) => panic!(Log::fatal(&format!("{}", why))),
                };
                if matched {
                    // A key.
                    self.buf_token_type = TokenType::Key;
                    Log::info_t(
                        "initial",
                        Table::default()
                            .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                            .char("ch", ch),
                    );
                    self.state = Some(LineMachineState::Key);
                } else {
                    self.buf_token_type = TokenType::Unimplemented;
                    Log::info_t(
                        "initial",
                        Table::default()
                            .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                            .char("ch", ch),
                    );
                }
            }
        }
        Log::info_t("End of initial", Table::default().char("ch", ch));
    }
}
impl fmt::Debug for LineParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.token_line)
    }
}

pub struct TokenLine {
    pub tokens: Vec<Token>,
}
impl Default for TokenLine {
    fn default() -> Self {
        TokenLine { tokens: Vec::new() }
    }
}
impl fmt::Debug for TokenLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&format!("{:?}", token));
        }
        write!(f, "{}", buf)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    /// }
    CloseCurlyBracket,
    /// ,
    Comma,
    Equals,
    Key,
    /// {
    LeftCurlyBracket,
    /// }
    RightCurlyBracket,
    /// #
    Sharp,
    /// '
    SingleQuotation,
    Unimplemented,
    /// Whitespace means tab (0x09 '\t') or space (0x20 ' ').
    WhiteSpace,
}

#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
}
impl Token {
    pub fn new(value: &str, type_: TokenType) -> Self {
        Token {
            value: value.to_string(),
            type_: type_,
        }
    }
}
impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}[{:?}]", self.value, self.type_)
    }
}
