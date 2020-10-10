//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::layer30::DocumentElement;
use crate::parser::syntax::{
    layer10::{CommentP, PResult, TableP},
    machine_state::BroadLineState,
    usize_to_i128, ArrayOfTableP, DocumentElementP, KeyValueP,
};
use crate::token::{Token, TokenType};
use casual_logger::Table;

impl Default for DocumentElementP {
    fn default() -> Self {
        DocumentElementP {
            array_of_table_p: None,
            buffer: None,
            comment_p: None,
            key_value_p: None,
            state: BroadLineState::First,
            table_p: None,
        }
    }
}
impl DocumentElementP {
    pub fn flush(&mut self) -> Option<DocumentElement> {
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
            BroadLineState::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::AfterComment => {
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::AfterKeyValue => match token.type_ {
                TokenType::EndOfLine => return PResult::End,
                _ => {
                    return PResult::Err(
                        self.log_table()
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            },
            BroadLineState::AfterLeftSquareBracket => match token.type_ {
                // `[`
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
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::ArrayOfTable => {
                let p = self.array_of_table_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_array_of_table(&child_m));
                            self.array_of_table_p = None;
                            self.state = BroadLineState::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            BroadLineState::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_comment(&child_m));
                            self.comment_p = None;
                            self.state = BroadLineState::AfterComment;
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
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            BroadLineState::First => match token.type_ {
                TokenType::EndOfLine => {
                    if let Some(_) = &self.comment_p {
                        return PResult::End;
                    }
                    if let Some(_) = &self.key_value_p {
                        return PResult::End;
                    }
                    self.buffer = Some(DocumentElement::EmptyLine);
                    self.state = BroadLineState::Finished;
                    return PResult::End;
                }
                // `[`
                TokenType::LeftSquareBracket => {
                    self.state = BroadLineState::AfterLeftSquareBracket;
                }
                TokenType::Key => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = BroadLineState::KeyValueSyntax;
                }
                // `#`
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
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            BroadLineState::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_key_value(&child_m));
                            self.key_value_p = None;
                            self.state = BroadLineState::AfterKeyValue;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            BroadLineState::Table => {
                return self.parse_table(token);
            }
            BroadLineState::Unimplemented => {
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
    fn parse_table(&mut self, token: &Token) -> PResult {
        let p = self.table_p.as_mut().unwrap();
        match p.parse(token) {
            PResult::End => {
                if let Some(child_m) = p.flush() {
                    self.buffer = Some(DocumentElement::from_table(&child_m));
                    self.table_p = None;
                    self.state = BroadLineState::AfterTable;
                    return PResult::End;
                } else {
                    return PResult::Err(
                        self.log_table()
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            } // Ignored it.
            PResult::Err(table) => {
                return PResult::Err(
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .sub_t("error", &table)
                        .clone(),
                );
            }
            PResult::Ongoing => PResult::Ongoing,
        }
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "DocumentElementP#parse")
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
