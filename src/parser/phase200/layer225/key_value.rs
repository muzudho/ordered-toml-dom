//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::LiteralString,
    layer220::{KeyValue, RightValue},
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, PResult, SingleQuotedStringP},
    layer220::{usize_to_i128, ArrayP},
    layer225::{InlineTableP, KeyValueP},
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
    AfterLeftCurlyBracket,
    AfterLeftSquareBracket,
    DoubleQuotedString,
    SingleQuotedString,
    End,
}

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            array_p: None,
            buffer: None,
            double_quoted_string_p: None,
            inline_table_p: None,
            single_quoted_string_p: None,
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
                        self.double_quoted_string_p = Some(DoubleQuotedStringP::new());
                        self.state = State::DoubleQuotedString;
                    }
                    // `literal`.
                    TokenType::KeyWithoutDot => {
                        // TODO true, false
                        self.buffer = Some(KeyValue::new(
                            &self.temp_key,
                            &RightValue::LiteralString(LiteralString::new(&token)),
                        ));
                        self.state = State::End;
                        return PResult::End;
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
            // After `{`.
            State::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValue::new(
                                &self.temp_key,
                                &RightValue::InlineTable(child_m),
                            ));
                            self.inline_table_p = None;
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
            // After `[`.
            State::AfterLeftSquareBracket => {
                let p = self.array_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer =
                                Some(KeyValue::new(&self.temp_key, &RightValue::Array(child_m)));
                            self.array_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "key_value.rs.215.")
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
                                        .str("via", "key_value.rs.224.")
                                        .int("column_number", usize_to_i128(token.column_number))
                                        .str("token", &format!("{:?}", token)),
                                )
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // `"abc"`.
            State::DoubleQuotedString => {
                let p = self.double_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValue::new(
                                &self.temp_key,
                                &RightValue::DoubleQuotedString(child_m),
                            ));
                            self.double_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "key_value.rs.252.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "key_value.rs.261.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // `'abc'`.
            State::SingleQuotedString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValue::new(
                                &self.temp_key,
                                &RightValue::SingleQuotedString(child_m),
                            ));
                            self.single_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "key_value.rs.291.")
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
                                        .str("via", "key_value.rs.300.")
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
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        if let Some(double_quoted_string_p) = &self.double_quoted_string_p {
            t.sub_t(
                "double_quoted_string",
                &double_quoted_string_p.log_snapshot(),
            );
        }
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.log_snapshot());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t(
                "single_quoted_string",
                &single_quoted_string_p.log_snapshot(),
            );
        }
        t
    }
}
