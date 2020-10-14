//! Key value syntax parser.  
//! キー値構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::RightValue,
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, LiteralStringP, PResult, SingleQuotedStringP},
    layer220::{usize_to_i128, ArrayP},
    layer225::{InlineTableP, RightValueP},
};
use crate::util::random_name;
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
                            return self.err_after_left_curly_bracket_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_after_left_curly_bracket_via(token, &mut table)
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
                            return self.err_after_left_square_bracket_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_after_left_square_bracket_via(token, &mut table)
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
                            return self.err_double_quoted_string_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_double_quoted_string_via(token, &mut table)
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
                        match p.parse(token) {
                            PResult::End => {
                                if let Some(child_m) = p.flush() {
                                    self.buffer = Some(RightValue::LiteralString(child_m));
                                    self.literal_string_p = None;
                                    self.state = State::End;
                                    return PResult::End;
                                } else {
                                    return self.err_parse_key_without_dot_empty_flush(token);
                                }
                            }
                            PResult::Err(mut table) => {
                                return self.err_parse_key_without_dot_via(token, &mut table)
                            }
                            PResult::Ongoing => {}
                        }
                    }
                }
            }
            // `abc`.
            State::LiteralString => {
                let p = self.literal_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(RightValue::LiteralString(child_m));
                            self.literal_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return self.err_parse_literal_string_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_parse_literal_string_via(token, &mut table)
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
                            self.buffer = Some(RightValue::SingleQuotedString(child_m));
                            self.single_quoted_string_p = None;
                            self.state = State::End;
                            return PResult::End;
                        } else {
                            return self.err_parse_single_quoted_string_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_parse_single_quoted_string_via(token, &mut table)
                    }
                    PResult::Ongoing => {}
                }
            }
            State::End => {
                return self.err_parse_end(token);
            }
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log_snapshot(&self) -> LogTable {
        let mut t = LogTable::default()
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

    /// Error message.  
    /// エラー・メッセージ。  
    fn err_after_left_curly_bracket_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_after_left_curly_bracket_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_after_left_curly_bracket_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str("via", "right_value.rs/err_after_left_curly_bracket_via")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_after_left_square_bracket_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_after_left_square_bracket_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_after_left_square_bracket_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str("via", "right_value.rs/err_after_left_square_bracket_via")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_double_quoted_string_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_double_quoted_string_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_double_quoted_string_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str(
                            "place_of_occurrence",
                            "right_value.rs/err_double_quoted_string_via",
                        )
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_key_without_dot_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_parse_key_without_dot_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_key_without_dot_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str(
                            "place_of_occurrence",
                            "right_value.rs/err_parse_key_without_dot_via",
                        )
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .sub_t("error", &table),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_literal_string_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_parse_literal_string_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_literal_string_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str(
                            "place_of_occurrence",
                            "right_value.rs/err_parse_literal_string_via",
                        )
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .sub_t("error", &table),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_single_quoted_string_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "right_value.rs/err_parse_single_quoted_string_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_single_quoted_string_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str("via", "right_value.rs/err_parse_single_quoted_string_via")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                )
                .clone(),
        )
    }
    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_end(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str("place_of_occurrence", "right_value.rs/err_parse_end")
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
}
