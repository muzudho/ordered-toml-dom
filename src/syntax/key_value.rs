//! Syntax parser.
//! 構文パーサー。

use crate::model::{KeyValueM, LiteralStringM, ValueM};
use crate::syntax::{
    machine_state::KeyValueState, ArrayP, InlineTableP, KeyValueP, SingleQuotedStringP,
    SyntaxParserResult,
};
use crate::token::{Token, TokenType};
use casual_logger::{Log, Table};

impl KeyValueP {
    pub fn new(key: &Token) -> Self {
        KeyValueP {
            state: KeyValueState::AfterKey,
            temp_key: key.clone(),
            buffer: None,
            inline_table_p: None,
            single_quoted_string_p: None,
            array_p: None,
        }
    }
    pub fn flush(&mut self) -> Option<KeyValueM> {
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
            KeyValueState::AfterKey => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/WhiteSpace",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    TokenType::Equals => {
                        self.state = KeyValueState::AfterEquals;
                        Log::trace_t(
                            "KeyValueP#parse/AfterKey/=",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
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
            KeyValueState::AfterEquals => {
                match token.type_ {
                    TokenType::WhiteSpace => {
                        Log::trace_t(
                            "KeyValueP#parse/After=/WhiteSpace",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    } //Ignored it.
                    TokenType::Key => {
                        // TODO true, false
                        self.buffer = Some(KeyValueM::new(
                            &self.temp_key,
                            &ValueM::LiteralString(LiteralStringM::new(&token)),
                        ));
                        self.state = KeyValueState::End;
                        Log::trace_t(
                            "KeyValueP#parse/After=/Key",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                        return SyntaxParserResult::End;
                    }
                    TokenType::LeftCurlyBracket => {
                        self.inline_table_p = Some(InlineTableP::default());
                        self.state = KeyValueState::AfterLeftCurlyBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/{",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::LeftSquareBracket => {
                        self.array_p = Some(ArrayP::default());
                        self.state = KeyValueState::AfterLeftSquareBracket;
                        Log::trace_t(
                            "KeyValueP#parse/After=/[",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
                    }
                    TokenType::SingleQuotation => {
                        self.single_quoted_string_p = Some(SingleQuotedStringP::new());
                        self.state = KeyValueState::SingleQuotedString;
                        Log::trace_t(
                            "KeyValueP#parse/After=/'",
                            Table::default().str("token", &format!("{:?}", token)),
                        );
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
            KeyValueState::AfterLeftCurlyBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After{",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                let p = self.inline_table_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValueM::new(
                                &self.temp_key,
                                &ValueM::InlineTable(child_m),
                            ));
                            self.inline_table_p = None;
                            self.state = KeyValueState::End;
                            return SyntaxParserResult::End;
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
            KeyValueState::AfterLeftSquareBracket => {
                Log::trace_t(
                    "KeyValueP#parse/After=/After[",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                let p = self.array_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer =
                                Some(KeyValueM::new(&self.temp_key, &ValueM::Array(child_m)));
                            self.array_p = None;
                            self.state = KeyValueState::End;
                            return SyntaxParserResult::End;
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
            KeyValueState::SingleQuotedString => {
                Log::trace_t(
                    "KeyValueP#parse/After=/'value'",
                    Table::default().str("token", &format!("{:?}", token)),
                );
                let p = self.single_quoted_string_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(KeyValueM::new(
                                &self.temp_key,
                                &ValueM::SingleQuotedString(child_m),
                            ));
                            self.single_quoted_string_p = None;
                            self.state = KeyValueState::End;
                            return SyntaxParserResult::End;
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
            KeyValueState::End => {
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
        let mut t = Table::default()
            .str("parser", "KeyValueP#parse")
            .str("state", &format!("{:?}", self.state))
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        if let Some(inline_table_p) = &self.inline_table_p {
            t.sub_t("inline_table", &inline_table_p.err_table());
        }
        if let Some(single_quoted_string_p) = &self.single_quoted_string_p {
            t.sub_t("single_quoted_string", &single_quoted_string_p.err_table());
        }
        t
    }
}
