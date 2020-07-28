//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::BroadLine;
use crate::syntax::usize_to_i128;
use crate::syntax::{
    machine_state::BroadLineState, ArrayOfTableP, BroadLineP, CommentP, KeyValueP,
    SyntaxParserResult, TableP,
};
use crate::token::{Token, TokenType};
use casual_logger::Table;

impl Default for BroadLineP {
    fn default() -> Self {
        BroadLineP {
            array_of_table_p: None,
            buffer: None,
            comment_p: None,
            key_value_p: None,
            state: BroadLineState::First,
            table_p: None,
        }
    }
}
impl BroadLineP {
    pub fn flush(&mut self) -> Option<BroadLine> {
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
            BroadLineState::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::AfterComment => {
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::AfterKeyValue => match token.type_ {
                TokenType::EndOfLine => return SyntaxParserResult::End,
                _ => {
                    return SyntaxParserResult::Err(
                        self.log_table()
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            },
            BroadLineState::AfterLeftSquareBracket => match token.type_ {
                TokenType::LeftSquareBracket => {
                    self.array_of_table_p = Some(ArrayOfTableP::new());
                    self.state = BroadLineState::ArrayOfTable;
                }
                _ => {
                    self.table_p = Some(TableP::new());
                    self.state = BroadLineState::Table;
                    return self.parse_table(token);
                }
            },
            BroadLineState::AfterTable => {
                // TODO 後ろにコメントがあるかも。
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::ArrayOfTable => {
                let p = self.array_of_table_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(BroadLine::from_array_of_table(&child_m));
                            self.array_of_table_p = None;
                            self.state = BroadLineState::AfterArrayOfTable;
                            return SyntaxParserResult::End;
                        } else {
                            return SyntaxParserResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            BroadLineState::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(BroadLine::from_comment(&child_m));
                            self.comment_p = None;
                            self.state = BroadLineState::AfterComment;
                            return SyntaxParserResult::End;
                        } else {
                            return SyntaxParserResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            BroadLineState::First => match token.type_ {
                TokenType::EndOfLine => {
                    if let Some(_) = &self.comment_p {
                        return SyntaxParserResult::End;
                    }
                    if let Some(_) = &self.key_value_p {
                        return SyntaxParserResult::End;
                    }
                    self.buffer = Some(BroadLine::EmptyLine);
                    self.state = BroadLineState::Finished;
                    return SyntaxParserResult::End;
                }
                TokenType::LeftSquareBracket => {
                    self.state = BroadLineState::AfterLeftSquareBracket;
                }
                TokenType::Key => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = BroadLineState::KeyValueSyntax;
                }
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = BroadLineState::CommentSyntax;
                }
                TokenType::WhiteSpace => {} // Ignored it.
                _ => {
                    self.state = BroadLineState::Unimplemented;
                }
            },
            BroadLineState::Finished => {
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(BroadLine::from_key_value(&child_m));
                            self.key_value_p = None;
                            self.state = BroadLineState::AfterKeyValue;
                            return SyntaxParserResult::End;
                        } else {
                            return SyntaxParserResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            BroadLineState::Table => {
                return self.parse_table(token);
            }
            BroadLineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }

        SyntaxParserResult::Ongoing
    }
    fn parse_table(&mut self, token: &Token) -> SyntaxParserResult {
        let p = self.table_p.as_mut().unwrap();
        match p.parse(token) {
            SyntaxParserResult::End => {
                if let Some(child_m) = p.flush() {
                    self.buffer = Some(BroadLine::from_table(&child_m));
                    self.table_p = None;
                    self.state = BroadLineState::AfterTable;
                    return SyntaxParserResult::End;
                } else {
                    return SyntaxParserResult::Err(
                        self.log_table()
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            } // Ignored it.
            SyntaxParserResult::Err(table) => {
                return SyntaxParserResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .sub_t("error", &table)
                        .clone(),
                );
            }
            SyntaxParserResult::Ongoing => SyntaxParserResult::Ongoing,
        }
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "BroadLineP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_p) = &self.comment_p {
            t.sub_t("comment", &comment_p.log_table());
        }
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.log_table());
        }
        t
    }
}
