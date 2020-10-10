//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer10::LiteralString,
    layer20::{KeyValue, RightValue},
};
use crate::parser::syntax::{
    layer10::{DoubleQuotedStringP, PResult, SingleQuotedStringP},
    layer20::{ArrayP, InlineTableP},
    machine_state::KeyValueState,
    usize_to_i128, KeyValueP,
};
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            array_p: None,
            buffer: None,
            double_quoted_string_p: None,
            inline_table_p: None,
            single_quoted_string_p: None,
            state: KeyValueState::AfterKey,
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
            KeyValueState::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/WhiteSpace",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    TokenType::Equals => {
                        self.state = KeyValueState::AfterEquals;
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/=",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            KeyValueState::AfterEquals => {
                match token.type_ {
                    TokenType::DoubleQuotation => {
                        self.double_quoted_string_p = Some(DoubleQuotedStringP::new());
                        self.state = KeyValueState::DoubleQuotedString;
                        Log::trace_t(
                            "KeyValueP#parse/After=/\"",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::Key => {
                        // TODO true, false
                        self.buffer = Some(KeyValue::new(
                            &self.temp_key,
                            &RightValue::LiteralString(LiteralString::new(&token)),
                        ));
                        self.state = KeyValueState::End;
                        Log::trace_t(
                            "KeyValueP#parse/After=/Key",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        return PResult::End;
                    }
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_p = Some(InlineTableP::default());
                        self.state = KeyValueState::AfterLeftCurlyBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/{",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(ArrayP::default());
                        self.state = KeyValueState::AfterLeftSquareBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/[",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(SingleQuotedStringP::new());
                        self.state = KeyValueState::SingleQuotedString;
                        Log::trace_t(
                            "KeyValueP#parse/After=/'",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/After=/WhiteSpace",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    _ => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            KeyValueState::AfterLeftCurlyBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After{",
                    self.log_table()
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
                            self.state = KeyValueState::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            KeyValueState::AfterLeftSquareBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After[",
                    self.log_table()
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
                            self.state = KeyValueState::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            KeyValueState::DoubleQuotedString => {
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
                            self.state = KeyValueState::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            KeyValueState::SingleQuotedString => {
                Log::trace_t(
                    "KeyValueP#parse/After=/'value'",
                    self.log_table()
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
                            self.state = KeyValueState::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            KeyValueState::End => {
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        PResult::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "KeyValueP#parse")
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        if let Some(double_quoted_string_p) = &self.double_quoted_string_p {
            t.sub_t("double_quoted_string", &double_quoted_string_p.log_table());
        }
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.log_table());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &single_quoted_string_p.log_table());
        }
        t
    }
}
