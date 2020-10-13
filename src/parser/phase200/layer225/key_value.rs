//! Key value syntax parser.  
//! キー値構文パーサー。  

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

/// Key value syntax machine state.  
/// キー値構文状態遷移。  
///
/// Example: `key = right_value`.  
#[derive(Debug)]
pub enum State {
    AfterKey,
    AfterEquals,
    RightValue,
    End,
}

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            buffer: None,
            right_value_p: None,
            state: State::AfterKey,
            temp_key: key.clone(),
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
            // After `literal`.
            State::AfterKey => {
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
                match token.type_ {
                    // `"`.
                    TokenType::DoubleQuotation => {
                        self.right_value_p = Some(RightValueP::default());
                        self.state = State::RightValue;
                        return self.right_value_p.as_mut().unwrap().parse(token);
                    }
                    // literal.
                    // TODO: 浮動小数点型の `.` や、 日付型に含まれる `:` なども拾えないか？
                    TokenType::KeyWithoutDot => {
                        self.right_value_p = Some(RightValueP::default());
                        self.state = State::RightValue;
                        return self.right_value_p.as_mut().unwrap().parse(token);
                    }
                    // `{`.
                    TokenType::LeftCurlyBracket => {
                        self.right_value_p = Some(RightValueP::default());
                        self.state = State::RightValue;
                        return self.right_value_p.as_mut().unwrap().parse(token);
                    }
                    // `[`.
                    TokenType::LeftSquareBracket => {
                        self.right_value_p = Some(RightValueP::default());
                        self.state = State::RightValue;
                        return self.right_value_p.as_mut().unwrap().parse(token);
                    }
                    // `'`.
                    TokenType::SingleQuotation => {
                        self.right_value_p = Some(RightValueP::default());
                        self.state = State::RightValue;
                        return self.right_value_p.as_mut().unwrap().parse(token);
                    }
                    TokenType::WhiteSpace => {} //Ignored it.
                    _ => {
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "key_value.rs.150.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // After `=`.
            State::RightValue => {
                let p = self.right_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValue::new(&self.temp_key, &child_m));
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
