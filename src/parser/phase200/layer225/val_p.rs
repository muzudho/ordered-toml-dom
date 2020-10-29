//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{layer110::TokenType, layer225::Val};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::{
    layer210::{BasicStringP, LiteralStringP, LiteralValueP, PResult},
    layer220::ArrayP,
    layer225::{InlineTableP, ValP},
};
use casual_logger::Table as LogTable;

/// Key value syntax machine state.  
/// キー値構文状態遷移。  
///
/// Example: `key = val`.  
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

impl Default for ValP {
    fn default() -> Self {
        ValP {
            array_p: None,
            buffer: None,
            basic_string_p: None,
            inline_table_p: None,
            literal_value_p: None,
            literal_string_p: None,
            state: State::First,
        }
    }
}
impl ValP {
    pub fn flush(&mut self) -> Option<Val> {
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
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            // After {.
            State::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Val::InlineTable(child_m));
                            self.inline_table_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "val.rs.68.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "val.rs.72.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // After [.
            State::AfterLeftSquareBracket => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Val::Array(child_m));
                            self.array_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "val.rs.88.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "val.rs.92.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // "abc"
            State::BasicString => {
                let p = self.basic_string_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Val::BasicString(child_m));
                            self.basic_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "val.rs.108.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "val.rs.112.")
                    }
                    PResult::Ongoing => {}
                }
            }
            State::First => {
                match token0.type_ {
                    // "
                    TokenType::DoubleQuotation => {
                        self.basic_string_p = Some(BasicStringP::new());
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
                        self.literal_string_p = Some(LiteralStringP::new());
                        self.state = State::LiteralString;
                    }
                    TokenType::WS => {} //Ignored it.
                    TokenType::Alpha
                    | TokenType::Digit
                    | TokenType::Hyphen
                    | TokenType::Underscore
                    | _ => {
                        self.literal_value_p = Some(LiteralValueP::default());
                        self.state = State::LiteralValue;
                        let p = self.literal_value_p.as_mut().unwrap();
                        match p.parse(&tokens) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.buffer = Some(Val::LiteralValue(child_m));
                                    self.literal_value_p = None;
                                    self.state = State::End;
                                    return PResult::End;
                                } else {
                                    return error(&mut self.log(), &tokens, "val.rs.152.");
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
                                    "val.rs.156.",
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
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Val::LiteralValue(child_m));
                            self.literal_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "val.rs.174.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "val.rs.178.")
                    }
                    PResult::Ongoing => {}
                }
            }
            // `'abc'`.
            State::LiteralString => {
                let p = self.literal_string_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Val::LiteralString(child_m));
                            self.literal_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "val.rs.194.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), &tokens, "val.rs.198.")
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return error(&mut self.log(), &tokens, "val.rs.204.");
            }
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", self.buffer))
            .clone();
        if let Some(p) = &self.basic_string_p {
            t.sub_t("basic_string_p", &p.log());
        }
        if let Some(p) = &self.inline_table_p {
            t.sub_t("inline_table", &p.log());
        }
        if let Some(p) = &self.literal_string_p {
            t.sub_t("single_quoted_string", &p.log());
        }
        t
    }
}
