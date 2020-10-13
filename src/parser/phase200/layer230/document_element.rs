//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer230::DocumentElement,
};
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable, PResult},
    layer220::usize_to_i128,
    layer225::KeyValueP,
    layer230::DocumentElementP,
};
use crate::util::random_name;
use casual_logger::Table;

/// Line syntax machine state.  
/// 行構文状態遷移。  
#[derive(Debug)]
pub enum State {
    AfterArrayOfTable,
    AfterComment,
    AfterKeyValue,
    AfterLeftSquareBracket,
    AfterTable,
    /// `[[name]]`
    HeaderOfArrayOfTable,
    /// `# comment`.
    CommentSyntax,
    Finished,
    First,
    /// `key = right_value`.
    KeyValueSyntax,
    /// `[name]`
    Table,
    Unimplemented,
}

impl Default for DocumentElementP {
    fn default() -> Self {
        DocumentElementP {
            header_p_of_array_of_table: None,
            buffer: None,
            comment_p: None,
            key_value_p: None,
            state: State::First,
            header_p_of_table: None,
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
            State::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.66.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            State::AfterComment => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.74.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            State::AfterKeyValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                // `,`
                TokenType::EndOfLine => return PResult::End,
                _ => {
                    return PResult::Err(
                        self.log_snapshot()
                            .str("place_of_occurrence", "document_element.rs.84.")
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            },
            State::AfterLeftSquareBracket => match token.type_ {
                // `[`
                TokenType::LeftSquareBracket => {
                    self.header_p_of_array_of_table = Some(HeaderPOfArrayOfTable::new());
                    self.state = State::HeaderOfArrayOfTable;
                }
                _ => {
                    self.header_p_of_table = Some(HeaderPOfTable::new());
                    self.state = State::Table;
                    return self.parse_header_of_table(token);
                }
            },
            State::AfterTable => {
                // TODO 後ろにコメントがあるかも。
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.106.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            State::HeaderOfArrayOfTable => {
                let p = self.header_p_of_array_of_table.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_header_of_array_of_table(&m));
                            self.header_p_of_array_of_table = None;
                            self.state = State::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "document_element.rs.123.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return PResult::Err(
                            table
                                .sub_t(
                                    &random_name(),
                                    self.log_snapshot()
                                        .str("via", "document_element.rs.132.")
                                        .int("column_number", usize_to_i128(token.column_number))
                                        .str("token", &format!("{:?}", token)),
                                )
                                .clone(),
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_comment(&m));
                            self.comment_p = None;
                            self.state = State::AfterComment;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "document_element.rs.153.")
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
                                        .str("via", "document_element.rs.162.")
                                        .int("column_number", usize_to_i128(token.column_number))
                                        .str("token", &format!("{:?}", token)),
                                )
                                .clone(),
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::First => match token.type_ {
                TokenType::EndOfLine => {
                    if let Some(_) = &self.comment_p {
                        return PResult::End;
                    }
                    if let Some(_) = &self.key_value_p {
                        return PResult::End;
                    }
                    self.buffer = Some(DocumentElement::EmptyLine);
                    self.state = State::Finished;
                    return PResult::End;
                }
                // `[`
                TokenType::LeftSquareBracket => {
                    self.state = State::AfterLeftSquareBracket;
                }
                // `abc`
                TokenType::KeyWithoutDot => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = State::KeyValueSyntax;
                }
                // `#`
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = State::CommentSyntax;
                }
                TokenType::WhiteSpace => {} // Ignored it.
                _ => {
                    self.state = State::Unimplemented;
                }
            },
            State::Finished => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.205.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            State::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_key_value(&m));
                            self.key_value_p = None;
                            self.state = State::AfterKeyValue;
                            return PResult::End;
                        } else {
                            return PResult::Err(
                                self.log_snapshot()
                                    .str("place_of_occurrence", "document_element.rs.222./KeyValueSyntax p.flush() is None.")
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return PResult::Err(
                            table
                                .sub_t(
                                    &random_name(),
                                    self.log_snapshot()
                                        .str("via", "document_element.rs.231.")
                                        .int("column_number", usize_to_i128(token.column_number))
                                        .str("token", &format!("{:?}", token))
                                        .sub_t("error", &table),
                                )
                                .clone(),
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Table => {
                return self.parse_header_of_table(token);
            }
            State::Unimplemented => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.246.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }

        PResult::Ongoing
    }
    /// Header of table.  
    /// テーブル・ヘッダー。  
    fn parse_header_of_table(&mut self, token: &Token) -> PResult {
        let p = self.header_p_of_table.as_mut().unwrap();
        match p.parse(token) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    self.buffer = Some(DocumentElement::from_header_of_table(&m));
                    self.header_p_of_table = None;
                    self.state = State::AfterTable;
                    return PResult::End;
                } else {
                    return PResult::Err(
                        self.log_snapshot()
                            .str("place_of_occurrence", "document_element.rs.269.")
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            } // Ignored it.
            PResult::Err(table) => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "document_element.rs.278.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .sub_t("error", &table)
                        .clone(),
                );
            }
            PResult::Ongoing => PResult::Ongoing,
        }
    }
    pub fn log_snapshot(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_p) = &self.comment_p {
            t.sub_t("comment_p", &comment_p.log_snapshot());
        }
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value_p", &key_value_p.log_snapshot());
        }
        t
    }
}
