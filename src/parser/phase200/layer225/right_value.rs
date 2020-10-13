//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::LiteralString,
    layer225::RightValue,
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, PResult, SingleQuotedStringP},
    layer220::{usize_to_i128, ArrayP},
    layer225::{InlineTableP, RightValueP},
};
use crate::util::random_name;
use casual_logger::Table;

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
    End,
}

impl Default for RightValueP {
    fn default() -> Self {
        RightValueP {
            array_p: None,
            double_quoted_string_p: None,
            inline_table_p: None,
            buffer: None,
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
    pub fn parse(&mut self, token: &Token) -> PResult {
        match self.state {
            // After `{`.
            State::AfterLeftCurlyBracket => {
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::InlineTable(child_m));
                            self.inline_table_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "right_value.rs.113.")
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
                                        .str("via", "right_value.rs.126.")
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
                            self.buffer = Some(RightValue::Array(child_m));
                            self.array_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "right_value.rs.149.")
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
                                        .str("via", "right_value.rs.162.")
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
                            self.buffer = Some(RightValue::DoubleQuotedString(child_m));
                            self.double_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "right_value.rs.185.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "right_value.rs.195.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
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
                    // literal.
                    // TODO: 浮動小数点型の `.` や、 日付型に含まれる `:` なども拾えないか？
                    TokenType::KeyWithoutDot => {
                        // TODO true, false
                        self.buffer = Some(RightValue::LiteralString(LiteralString::new(&token)));
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
                                .str("place_of_occurrence", "right_value.rs.84.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // `'abc'`.
            State::SingleQuotedString => {
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::SingleQuotedString(child_m));
                            self.single_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "right_value.rs.218.")
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
                                        .str("via", "right_value.rs.231.")
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
                        .str("place_of_occurrence", "right_value.rs.244.")
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
