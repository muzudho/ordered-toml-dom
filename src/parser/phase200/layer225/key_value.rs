//! Key value syntax parser.  
//! キー値構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // key = right_value
//! ```

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::KeyValue,
};
use crate::parser::phase200::{
    layer210::PResult,
    layer220::usize_to_i128,
    layer225::{KeyValueP, RightValueP},
};
use crate::util::random_name;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug)]
pub enum State {
    // After key.
    // キーの後。
    First,
    AfterEquals,
    RightValue,
    End,
}

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            buffer: None,
            key: key.clone(),
            right_value_p: None,
            state: State::First,
        }
    }

    pub fn flush(&mut self) -> Option<KeyValue> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }

    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> PResult {
        match self.state {
            // After key.
            State::First => {
                match token.type_ {
                    TokenType::WhiteSpace => {} //Ignored it.
                    // `=`.
                    TokenType::Equals => {
                        self.state = State::AfterEquals;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "key_value.rs.75.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // After `=`.
            State::AfterEquals => {
                self.right_value_p = Some(RightValueP::default());
                self.state = State::RightValue;
            }
            // After `=`.
            State::RightValue => {
                let p = self.right_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValue::new(&self.key, &child_m));
                            self.right_value_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "key_value.rs.178.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(mut table) => {
                        return PResult::Err(
                            table
                                .sub_t(
                                    &random_name(),
                                    self.log_snapshot()
                                        .str("via", "key_value.rs.187.")
                                        .int("column_number", usize_to_i128(token.column_number))
                                        .str("token", &format!("{:?}", token)),
                                )
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "key_value.rs.312.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        PResult::Ongoing
    }
    pub fn log_snapshot(&self) -> Table {
        let mut t = Table::default()
            .str("buffer", &format!("{:?}", &self.buffer))
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(right_value_p) = &self.right_value_p {
            t.sub_t("right_value_p", &right_value_p.log_snapshot());
        }
        t
    }
}
