//! 単語単位に分けます。
use crate::token::{Token, TokenLine, TokenType};
use crate::RE_KEY;
use casual_logger::Log;
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
pub struct LineLexicalParser {
    state: Option<LineMachineState>,
    product: TokenLine,
    buf_token_type: TokenType,
    buf: String,
}
impl Default for LineLexicalParser {
    fn default() -> Self {
        LineLexicalParser {
            state: None,
            product: TokenLine::default(),
            buf_token_type: TokenType::WhiteSpace,
            buf: String::new(),
        }
    }
}
impl LineLexicalParser {
    pub fn product(&self) -> &TokenLine {
        &self.product
    }
    pub fn parse_line(&mut self, line: &str) {
        let ch_vec: Vec<char> = line.chars().collect();
        for ch in ch_vec {
            if let Some(state) = &self.state {
                match state {
                    LineMachineState::WhiteSpace => {
                        // 最初に出てくる文字まで飛ばします。
                        match ch {
                            '\t' | ' ' => {
                                self.buf.push(ch);
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
                        } else {
                            self.flush();
                            self.initial(ch);
                        }
                    }
                }
            } else {
                self.flush();
                self.initial(ch);
            }
        }
        // Log::info("End of line.");
        self.flush();
    }
    /// Flush.
    fn flush(&mut self) {
        if !self.buf.is_empty() {
            self.product
                .tokens
                .push(Token::new(&self.buf, self.buf_token_type));
            // Log::info_t("Flush", Table::default().str("buf", &self.buf));
            self.buf.clear();
            self.state = None;
        }
    }
    /// 最初の文字
    fn initial(&mut self, ch: char) {
        self.buf.push(ch);
        // Log::info_t("Begin of initial", Table::default().char("ch", ch));
        match ch {
            '\t' | ' ' => {
                self.buf_token_type = TokenType::WhiteSpace;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
                self.state = Some(LineMachineState::WhiteSpace);
            }
            ',' => {
                self.buf_token_type = TokenType::Comma;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '=' => {
                self.buf_token_type = TokenType::Equals;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '{' => {
                self.buf_token_type = TokenType::LeftCurlyBracket;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '[' => {
                self.buf_token_type = TokenType::LeftSquareBracket;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '}' => {
                self.buf_token_type = TokenType::RightCurlyBracket;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            ']' => {
                self.buf_token_type = TokenType::RightSquareBracket;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '#' => {
                self.buf_token_type = TokenType::Sharp;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            '\'' => {
                self.buf_token_type = TokenType::SingleQuotation;
                /*
                Log::info_t(
                    "initial",
                    Table::default()
                        .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                        .char("ch", ch),
                );
                */
            }
            _ => {
                let matched = match RE_KEY.lock() {
                    Ok(re_key) => re_key.is_match(&ch.to_string()),
                    Err(why) => panic!(Log::fatal(&format!("{}", why))),
                };
                if matched {
                    // A key.
                    self.buf_token_type = TokenType::Key;
                    /*
                    Log::info_t(
                        "initial",
                        Table::default()
                            .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                            .char("ch", ch),
                    );
                    */
                    self.state = Some(LineMachineState::Key);
                } else {
                    self.buf_token_type = TokenType::Unimplemented;
                    /*
                    Log::info_t(
                        "initial",
                        Table::default()
                            .str("buf_token_type", &format!("{:?}", self.buf_token_type))
                            .char("ch", ch),
                    );
                    */
                }
            }
        }
        // Log::info_t("End of initial", Table::default().char("ch", ch));
    }
}
impl fmt::Debug for LineLexicalParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.product)
    }
}
