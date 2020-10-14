//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::RightValue,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::{DoubleQuotedStringP, LiteralStringP, PResult, SingleQuotedStringP},
    layer220::ArrayP,
    layer225::{InlineTableP, RightValueP},
};
use casual_logger::Table as LogTable;

/// Key value syntax machine state.  
/// キー値構文状態遷移。  
///
/// Example: `key = right_value`.  
#[derive(Debug)]
pub enum State {
    AfterLeftCurlyBracket,
    AfterLeftSquareBracket,
    DoubleQuotedString,
    First,
    SingleQuotedString,
    LiteralString,
    End,
}

impl Default for RightValueP {
    fn default() -> Self {
        RightValueP {
            array_p: None,
            buffer: None,
            double_quoted_string_p: None,
            inline_table_p: None,
            literal_string_p: None,
            single_quoted_string_p: None,
            state: State::First,
        }
    }
}
impl RightValueP {
    pub fn flush(&mut self) -> Option<RightValue> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }

    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_token: Option<&Token>, token: &Token) -> PResult {
        match self.state {
            // After `{`.
            State::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::InlineTable(child_m));
                            self.inline_table_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "right_value.rs.68.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), token, "right_value.rs.72.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `[`.
            State::AfterLeftSquareBracket => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::Array(child_m));
                            self.array_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "right_value.rs.88.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), token, "right_value.rs.92.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // `"abc"`.
            State::DoubleQuotedString => {
                let p = self.double_quoted_string_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::DoubleQuotedString(child_m));
                            self.double_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "right_value.rs.108.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), token, "right_value.rs.112.")
                    }
                    PResult::Ongoing => {}
                }
            }
            State::First => {
                match token.type_ {
                    // `"`.
                    TokenType::DoubleQuotation => {
                        self.double_quoted_string_p = Some(DoubleQuotedStringP::new());
                        self.state = State::DoubleQuotedString;
                    }
                    // `{`.
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_p = Some(InlineTableP::default());
                        self.state = State::AfterLeftCurlyBracket;
                    }
                    // `[`.
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(ArrayP::default());
                        self.state = State::AfterLeftSquareBracket;
                    }
                    // `'`.
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(SingleQuotedStringP::new());
                        self.state = State::SingleQuotedString;
                    }
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::KeyWithoutDot | _ => {
                        self.literal_string_p = Some(LiteralStringP::new());
                        self.state = State::LiteralString;
                        let p = self.literal_string_p.as_mut().unwrap();
                        match p.parse(look_ahead_token, token) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.buffer = Some(RightValue::LiteralString(child_m));
                                    self.literal_string_p = None;
                                    self.state = State::End;
                                    return PResult::End;
                                } else {
                                    return error(&mut self.log(), token, "right_value.rs.152.");
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    token,
                                    "right_value.rs.156.",
                                )
                            }
                            PResult::Ongoing => {}
                        }
                    }
                }
            }
            // `abc`.
            State::LiteralString => {
                let p = self.literal_string_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::LiteralString(child_m));
                            self.literal_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "right_value.rs.174.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), token, "right_value.rs.178.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // `'abc'`.
            State::SingleQuotedString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(look_ahead_token, token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::SingleQuotedString(child_m));
                            self.single_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), token, "right_value.rs.194.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), token, "right_value.rs.198.")
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return error(&mut self.log(), token, "right_value.rs.204.");
            }
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        if let Some(double_quoted_string_p) = &self.double_quoted_string_p {
            t.sub_t("double_quoted_string", &double_quoted_string_p.log());
        }
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.log());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &single_quoted_string_p.log());
        }
        t
    }
}
