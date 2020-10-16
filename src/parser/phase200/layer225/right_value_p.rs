//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::RightValue,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::{BasicStringP, LiteralStringP, LiteralValueP, PResult},
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
    BasicString,
    First,
    LiteralString,
    LiteralValue,
    End,
}

impl Default for RightValueP {
    fn default() -> Self {
        RightValueP {
            array_p: None,
            buffer: None,
            double_quoted_string_p: None,
            inline_table_p: None,
            literal_value_p: None,
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

    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            // After `{`.
            State::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::InlineTable(child_m));
                            self.inline_table_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "right_value.rs.68.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), tokens, "right_value.rs.72.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `[`.
            State::AfterLeftSquareBracket => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::Array(child_m));
                            self.array_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "right_value.rs.88.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), tokens, "right_value.rs.92.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // `"abc"`.
            State::BasicString => {
                let p = self.double_quoted_string_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::BasicString(child_m));
                            self.double_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "right_value.rs.108.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "right_value.rs.112.",
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::First => {
                match token0.type_ {
                    // `"`.
                    TokenType::DoubleQuotation => {
                        self.double_quoted_string_p = Some(BasicStringP::new());
                        self.state = State::BasicString;
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
                        self.single_quoted_string_p = Some(LiteralStringP::new());
                        self.state = State::LiteralString;
                    }
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::KeyWithoutDot | _ => {
                        self.literal_value_p = Some(LiteralValueP::new());
                        self.state = State::LiteralValue;
                        let p = self.literal_value_p.as_mut().unwrap();
                        match p.parse(tokens) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.buffer = Some(RightValue::LiteralValue(child_m));
                                    self.literal_value_p = None;
                                    self.state = State::End;
                                    return PResult::End;
                                } else {
                                    return error(&mut self.log(), tokens, "right_value.rs.152.");
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    tokens,
                                    "right_value.rs.156.",
                                )
                            }
                            PResult::Ongoing => {}
                        }
                    }
                }
            }
            // `abc`.
            State::LiteralValue => {
                let p = self.literal_value_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::LiteralValue(child_m));
                            self.literal_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "right_value.rs.174.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "right_value.rs.178.",
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // `'abc'`.
            State::LiteralString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::LiteralString(child_m));
                            self.single_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "right_value.rs.194.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "right_value.rs.198.",
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return error(&mut self.log(), tokens, "right_value.rs.204.");
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
            t.sub_t("basic_strings", &double_quoted_string_p.log());
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
