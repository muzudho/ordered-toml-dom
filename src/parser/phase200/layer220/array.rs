//! Array syntax parser.  
//! 配列構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // [ 1, 2, 3 ]
//! ```

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::LiteralString,
    layer220::Array,
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, PResult, SingleQuotedStringP},
    layer220::{usize_to_i128, ArrayP},
};
use casual_logger::{Log, Table};

/// Array syntax machine state.  
/// 配列構文状態遷移。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone, Debug)]
pub enum State {
    /// `[ "a",` の次。
    AfterCommaBefindDoubleQuotedString,
    /// [ か , の次。
    AfterDoubleQuotedString,
    AfterLeftSquareBracket,
    AfterSingleQuotedString,
    /// , か ] を待ちます。
    AfterItem,
    DoubleQuotedString,
    End,
    SingleQuotedString,
}

impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            buffer: None,
            double_quoted_string_p: None,
            single_quoted_string_p: None,
            state: State::AfterLeftSquareBracket,
        }
    }
}
impl ArrayP {
    pub fn flush(&mut self) -> Option<Array> {
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
            // `[ "a",` の次。
            State::AfterCommaBefindDoubleQuotedString => {
                match token.type_ {
                    TokenType::DoubleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse/After[/\"",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.double_quoted_string_p = Some(Box::new(DoubleQuotedStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse/After[/WhiteSpace",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
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
            // After `"`.
            State::AfterDoubleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/After\"value\"",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = State::AfterCommaBefindDoubleQuotedString;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
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
            // After `[`.
            State::AfterLeftSquareBracket => {
                match token.type_ {
                    TokenType::DoubleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse/After[/\"",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.double_quoted_string_p = Some(Box::new(DoubleQuotedStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    TokenType::KeyWithoutDot => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralString::new(token));
                        self.state = State::AfterItem;
                        Log::trace_t(
                            "ArrayP#parse/After[/Key",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .str("buffer", &format!("{:?}", self.buffer)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse/After[/'",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = State::SingleQuotedString;
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse/After[/WhiteSpace",
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
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
            State::AfterItem => {
                Log::trace_t(
                    "ArrayP#parse/After*",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::Comma => {
                        self.state = State::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
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
            // After `'`.
            State::AfterSingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/After'value'",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = State::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
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
            // "dog".
            State::DoubleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/\"value\"",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                let p = self.double_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_double_quote_string(&child_m);
                            self.double_quoted_string_p = None;
                            self.state = State::AfterDoubleQuotedString;
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
            State::End => {
                Log::trace_t(
                    "ArrayP#parse/End",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            // `'C:\temp'`.
            State::SingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/'value'",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_single_quote_string(&child_m);
                            self.single_quoted_string_p = None;
                            self.state = State::AfterSingleQuotedString;
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
        }
        PResult::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state));

        if let Some(p) = &self.double_quoted_string_p {
            t.sub_t("double_quoted_string", &p.log_table());
        }
        if let Some(p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &p.log_table());
        }

        t
    }
}
