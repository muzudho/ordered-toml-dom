//! Key value syntax parser.  
//! キー値構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // key = right_value
//! ```

use crate::model::{
    layer110::{Token, TokenType},
    layer225::KeyValue,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::{KeyP, PResult},
    layer225::{KeyValueP, RightValueP},
};
use casual_logger::Table as LogTable;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug)]
pub enum State {
    AfterEquals,
    // After key.
    // キーの後。
    BeforeEqual,
    End,
    First,
    RightValue,
}

impl KeyValueP {
    pub fn new() -> Self {
        KeyValueP {
            key_buffer: None,
            right_value_buffer: None,
            key_p: Some(KeyP::default()),
            right_value_p: None,
            state: State::First,
        }
    }

    pub fn flush(&mut self) -> Option<KeyValue> {
        let m = if let Some(key) = &self.key_buffer {
            if let Some(right_value) = &self.right_value_buffer {
                Some(KeyValue::new(&key, &right_value))
            } else {
                panic!("key_value_p.rs.53.")
            }
        } else {
            panic!("key_value_p.rs.56.")
        };
        self.key_buffer = None;
        self.right_value_buffer = None;
        m
    }

    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            // After `=`.
            State::AfterEquals => {
                self.right_value_p = Some(RightValueP::default());
                self.state = State::RightValue;
            }
            // After key.
            State::BeforeEqual => {
                match token0.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    // `=`.
                    TokenType::Equals => {
                        self.state = State::AfterEquals;
                    }
                    _ => return error(&mut self.log(), tokens, "key_value.rs.65."),
                }
            }
            State::First => {
                match token0.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    TokenType::Alphabet
                    | TokenType::Numeral
                    | TokenType::Hyphen
                    | TokenType::Underscore => {
                        let p = self.key_p.as_mut().unwrap();
                        match p.parse(tokens) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.key_buffer = Some(child_m);
                                    self.key_p = None;
                                    self.state = State::BeforeEqual;
                                } else {
                                    return error(&mut self.log(), tokens, "key_value.rs.84.");
                                }
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    tokens,
                                    "key_value.rs.84.",
                                );
                            }
                            PResult::Ongoing => {}
                        }
                    }
                    _ => return error(&mut self.log(), tokens, "key_value.rs.65."),
                }
            }
            // After `=`.
            State::RightValue => {
                let p = self.right_value_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.right_value_buffer = Some(child_m);
                            self.right_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "key_value.rs.84.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(&mut table, &mut self.log(), tokens, "key_value.rs.88.");
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => return error(&mut self.log(), tokens, "key_value.rs.93."),
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("key_buffer", &format!("{:?}", &self.key_buffer))
            .str(
                "right_value_buffer",
                &format!("{:?}", &self.right_value_buffer),
            )
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(right_value_p) = &self.right_value_p {
            t.sub_t("right_value_p", &right_value_p.log());
        }
        t
    }
}
