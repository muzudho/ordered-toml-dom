//! Syntax parser.
//! 構文パーサー。

use crate::model::{ArrayM, LiteralStringM};
use crate::syntax::{machine_state::ArrayState, ArrayP, SingleQuotedStringP, SyntaxParserResult};
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

impl Default for ArrayP {
    fn default() -> Self {
        ArrayP {
            state: ArrayState::AfterLeftSquareBracket,
            buffer: None,
            single_quoted_string_p: None,
        }
    }
}
impl ArrayP {
    pub fn flush(&mut self) -> Option<ArrayM> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> SyntaxParserResult {
        match self.state {
            ArrayState::AfterLeftSquareBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "ArrayP#parse/After[/WhiteSpace",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    } // Ignore it.
                    TokenType::Key => {
                        // TODO 数字なら正しいが、リテラル文字列だと間違い。キー・バリューかもしれない。
                        if let None = self.buffer {
                            self.buffer = Some(ArrayM::default());
                        }
                        let m = self.buffer.as_mut().unwrap();
                        m.push_literal_string(&LiteralStringM::new(token));
                        self.state = ArrayState::AfterItem;
                        Log::trace_t(
                            "ArrayP#parse/After[/Key",
                            Table::default()
                                .str("token", &format!("{:?}", token))
                                .str("buffer", &format!("{:?}", self.buffer)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        Log::trace_t(
                            "ArrayP#parse/After[/'",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                        self.single_quoted_string_p = Some(Box::new(SingleQuotedStringP::new()));
                        self.state = ArrayState::SingleQuotedString;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            ArrayState::SingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/'value'",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            if let None = self.buffer {
                                self.buffer = Some(ArrayM::default());
                            }
                            let m = self.buffer.as_mut().unwrap();
                            m.push_single_quote_string(&child_m);
                            self.single_quoted_string_p = None;
                            self.state = ArrayState::AfterSingleQuotedString;
                        } else {
                            return SyntaxParserResult::Err(
                                self.err_table()
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            ArrayState::AfterItem => {
                Log::trace_t(
                    "ArrayP#parse/After*",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::Comma => {
                        self.state = ArrayState::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = ArrayState::End;
                        return SyntaxParserResult::End;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            ArrayState::AfterSingleQuotedString => {
                Log::trace_t(
                    "ArrayP#parse/After'value'",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Comma => {
                        self.state = ArrayState::AfterLeftSquareBracket;
                    }
                    TokenType::RightSquareBracket => {
                        self.state = ArrayState::End;
                        return SyntaxParserResult::End;
                    }
                    _ => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        )
                    }
                }
            }
            ArrayState::End => {
                Log::trace_t(
                    "ArrayP#parse/End",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }
        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("parser", "ArrayP#parse")
            .str("state", &format!("{:?}", self.state));

        if let Some(p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &p.err_table());
        }
        t
    }
}
