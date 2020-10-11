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
    /// After `[array]`.
    AfterArray,
    /// After `[],`.
    AfterCommaBehindArray,
    /// After `[ "a",`.
    AfterCommaBefindQuotedString,
    /// After `[ true,`.
    AfterCommaBehindKeyWithoutDot,
    /// After `[` or `,`.
    AfterDoubleQuotedString,
    /// After `[`.
    First,
    AfterSingleQuotedString,
    /// `[ true` , か ] を待ちます。
    AfterKeyWithoutDot,
    /// After `[`.
    Array,
    DoubleQuotedString,
    End,
    SingleQuotedString,
}

impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            buffer: None,
            array_p: None,
            double_quoted_string_p: None,
            single_quoted_string_p: None,
            state: State::First,
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
            // After `]`.
            State::AfterArray => {
                Log::trace_t(
                    "ArrayP#parse| [array] -> this",
                    self.log_table("code.76.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    // `,`.
                    TokenType::Comma => {
                        self.state = State::AfterCommaBehindArray;
                    }
                    // `]`.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.93.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // After `[],`.
            State::AfterCommaBehindArray => {
                match token.type_ {
                    // `[`.
                    TokenType::LeftSquareBracket => {
                        Log::trace_t(
                            "ArrayP#parse| [], -> [",
                            self.log_table("code.108.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse| [], -> WhiteSpace",
                            self.log_table("code.118.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    // `]`.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.130.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // `"a",` の次。
            State::AfterCommaBefindQuotedString => {
                match token.type_ {
                    TokenType::DoubleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse| \"a\", -> \"",
                            self.log_table("code.144.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.double_quoted_string_p = Some(Box::new(DoubleQuotedStringP::new()));
                        self.state = State::DoubleQuotedString;
                    }
                    TokenType::SingleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse| \"a\", -> '",
                            self.log_table("code.154.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = State::SingleQuotedString;
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse| \"a\", -> WhiteSpace",
                            self.log_table("code.164.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    // `]`.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.176.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // After `literal,`.
            State::AfterCommaBehindKeyWithoutDot => {
                match token.type_ {
                    TokenType::KeyWithoutDot => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(Array::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralString::new(token));
                        self.state = State::AfterKeyWithoutDot;
                        Log::trace_t(
                            "ArrayP#parse| [ literal, -> KeyWithoutDot",
                            self.log_table("code.197.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .str("buffer", &format!("{:?}", self.buffer)),
                        );
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse| [ -> WhiteSpace",
                            self.log_table("code.206.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    // `]`.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.218.")
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
                    "ArrayP#parse| \"value\" -> this",
                    self.log_table("code.230.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = State::AfterCommaBefindQuotedString;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.245.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            // `[array]`.
            State::Array => {
                Log::trace_t(
                    "ArrayP#parse| [array]",
                    self.log_table("code.257.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                let p = self.array_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(Array::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_array(&child_m);
                        } else {
                            // Empty array.
                        }
                        self.array_p = None;
                        self.state = State::AfterArray;
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.283.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `[`.
            State::First => {
                match token.type_ {
                    // `]`. Empty array.
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    // `[`. Recursive.
                    TokenType::LeftSquareBracket => {
                        Log::trace_t(
                            "ArrayP#parse| [ -> [",
                            self.log_table("code.305.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.array_p = Some(Box::new(ArrayP::default()));
                        self.state = State::Array;
                    }
                    TokenType::DoubleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse| [ -> \"",
                            self.log_table("code.315.")
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
                        self.state = State::AfterKeyWithoutDot;
                        Log::trace_t(
                            "ArrayP#parse| [ -> KeyWithoutDot",
                            self.log_table("code.332.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .str("buffer", &format!("{:?}", self.buffer)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse| [ -> /'",
                            self.log_table("code.341.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = State::SingleQuotedString;
                    }
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse| [ -> WhiteSpace",
                            self.log_table("code.351.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    _ => {
                        return PResult::Err(
                            self.log_table("code.358.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            State::AfterKeyWithoutDot => {
                Log::trace_t(
                    "ArrayP#parse| KeyWithoutDot -> this",
                    self.log_table("code.369.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::Comma => {
                        self.state = State::AfterCommaBehindKeyWithoutDot;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.383.")
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
                    "ArrayP#parse| 'value' -> this",
                    self.log_table("code.395.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = State::AfterCommaBefindQuotedString;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return PResult::Err(
                            self.log_table("code.410.")
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
                    "ArrayP#parse| \"value\"",
                    self.log_table("code.422.")
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
                                self.log_table("code.439.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.448.")
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
                    "ArrayP#parse| End",
                    self.log_table("code.461.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                );
                return PResult::Err(
                    self.log_table("code.466.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            // `'C:\temp'`.
            State::SingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse| 'value'",
                    self.log_table("code.476.")
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
                                self.log_table("code.493.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table("code.502.")
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
    pub fn log_table(&self, code_location: &str) -> Table {
        let mut t = Table::default()
            .str("code_location", code_location)
            .str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();

        if let Some(p) = &self.double_quoted_string_p {
            t.sub_t("double_quoted_string_p", &p.log_table("code.517."));
        }
        if let Some(p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string_p", &p.log_table("code.520."));
        }
        if let Some(p) = &self.array_p {
            t.sub_t("array_p", &p.log_table("code.528."));
        }

        t
    }
}
