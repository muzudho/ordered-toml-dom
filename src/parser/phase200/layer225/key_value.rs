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
use casual_logger::{Log, Table};

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
            State::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/WhiteSpace",
                            self.log_table("code.59.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    TokenType::Equals => {
                        self.state = State::AfterEquals;
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/=",
                            self.log_table("code.68.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.75.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            State::AfterEquals => {
                match token.type_ {
                    TokenType::DoubleQuotation => {
                        self.double_quoted_string_p = Some(DoubleQuotedStringP::new());
                        self.state = State::DoubleQuotedString;
                        Log::trace_t(
                            "KeyValueP#parse/After=/\"",
                            self.log_table("code.90.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::KeyWithoutDot => {
                        // TODO true, false
                        self.buffer = Some(KeyValue::new(
                            &self.temp_key,
                            &RightValue::LiteralString(LiteralString::new(&token)),
                        ));
                        self.state = State::End;
                        Log::trace_t(
                            "KeyValueP#parse/After=/Key",
                            self.log_table("code.104.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        return PResult::End;
                    }
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_p = Some(InlineTableP::default());
                        self.state = State::AfterLeftCurlyBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/{",
                            self.log_table("code.115.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(ArrayP::default());
                        self.state = State::AfterLeftSquareBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/[",
                            self.log_table("code.125.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(SingleQuotedStringP::new());
                        self.state = State::SingleQuotedString;
                        Log::trace_t(
                            "KeyValueP#parse/After=/'",
                            self.log_table("code.135.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/After=/WhiteSpace",
                            self.log_table("code.143.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    _ => {
                        return PResult::Err(
                            self.log_table("code.150.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            State::AfterLeftCurlyBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After{",
                    self.log_table("code.161.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
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
                                self.log_table("code.178.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("key_value.rs.187.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::AfterLeftSquareBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After[",
                    self.log_table("code.200.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
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
                                self.log_table("code.215.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.224.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::DoubleQuotedString => {
                Log::trace_t(
                    "KeyValueP#parse/After=/\"value\"",
                    Table::default().str("token", &format!("{:?}", token)),
                );
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
                                self.log_table("code.252.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.261.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::SingleQuotedString => {
                Log::trace_t(
                    "KeyValueP#parse/After=/'value'",
                    self.log_table("code.274.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
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
                                self.log_table("code.291.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.300.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return PResult::Err(
                    self.log_table("code.312.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        PResult::Ongoing
    }
    pub fn log_table(&self, place_of_occurrence: &str) -> Table {
        let mut t = Table::default()
            .str("place_of_occurrence", place_of_occurrence)
            .str("parser", "KeyValueP#parse")
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        if let Some(double_quoted_string_p) = &self.double_quoted_string_p {
            t.sub_t(
                "double_quoted_string",
                &double_quoted_string_p.log_table(place_of_occurrence),
            );
        }
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.log_snapshot());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t(
                "single_quoted_string",
                &single_quoted_string_p.log_table(place_of_occurrence),
            );
        }
        t
    }
}
