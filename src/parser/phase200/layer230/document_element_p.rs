//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer230::DocumentElement,
};
use crate::parser::phase200::{
    error, error_via,
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable, PResult},
    layer225::KeyValueP,
    layer230::DocumentElementP,
};
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

    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            State::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), tokens, "document_element.rs.66.");
            }
            State::AfterComment => {
                return error(&mut self.log(), tokens, "document_element.rs.74.");
            }
            State::AfterKeyValue => match token0.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                // `,`
                TokenType::EndOfLine => return PResult::End,
                _ => {
                    return error(&mut self.log(), tokens, "document_element.rs.84.");
                }
            },
            State::AfterLeftSquareBracket => match token0.type_ {
                // `[`
                TokenType::LeftSquareBracket => {
                    self.header_p_of_array_of_table = Some(HeaderPOfArrayOfTable::new());
                    self.state = State::HeaderOfArrayOfTable;
                }
                _ => {
                    self.header_p_of_table = Some(HeaderPOfTable::new());
                    self.state = State::Table;
                    return self.parse_header_of_table(tokens);
                }
            },
            State::AfterTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), tokens, "document_element.rs.106.");
            }
            State::HeaderOfArrayOfTable => {
                let p = self.header_p_of_array_of_table.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_header_of_array_of_table(&m));
                            self.header_p_of_array_of_table = None;
                            self.state = State::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "document_element.rs.123.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "document_element.rs.132.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_comment(&m));
                            self.comment_p = None;
                            self.state = State::AfterComment;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "document_element.rs.153.");
                        }
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "document_element.rs.162.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::First => match token0.type_ {
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
                // `abc`
                TokenType::KeyWithoutDot => {
                    self.key_value_p = Some(KeyValueP::new());
                    match self.key_value_p.as_mut().unwrap().parse(tokens) {
                        PResult::End => {
                            // 1トークンでは終わらないから。
                            return error(&mut self.log(), tokens, "document_element.rs.164.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                tokens,
                                "document_element.rs.171.",
                            )
                        }
                        PResult::Ongoing => {}
                    }
                    self.state = State::KeyValueSyntax;
                }
                // `[`
                TokenType::LeftSquareBracket => {
                    self.state = State::AfterLeftSquareBracket;
                }
                // `#`
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = State::CommentSyntax;
                }
                TokenType::WhiteSpace => {} // Ignored it.
                _ => {
                    return error(&mut self.log(), tokens, "document_element.rs.246.");
                }
            },
            State::Finished => {
                return error(&mut self.log(), tokens, "document_element.rs.205.");
            }
            State::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(DocumentElement::from_key_value(&m));
                            self.key_value_p = None;
                            self.state = State::AfterKeyValue;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), tokens, "document_element.rs.222.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            tokens,
                            "document_element.rs.231.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Table => {
                return self.parse_header_of_table(tokens);
            }
        }

        PResult::Ongoing
    }
    /// Header of table.  
    /// テーブル・ヘッダー。  
    ///
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    fn parse_header_of_table(
        &mut self,
        tokens: (Option<&Token>, Option<&Token>, Option<&Token>),
    ) -> PResult {
        let p = self.header_p_of_table.as_mut().unwrap();
        match p.parse(tokens) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    self.buffer = Some(DocumentElement::from_header_of_table(&m));
                    self.header_p_of_table = None;
                    self.state = State::AfterTable;
                    return PResult::End;
                } else {
                    return error(&mut self.log(), tokens, "document_element.rs.269.");
                }
            } // Ignored it.
            PResult::Err(mut table) => {
                return error_via(
                    &mut table,
                    &mut self.log(),
                    tokens,
                    "document_element.rs.278.",
                );
            }
            PResult::Ongoing => PResult::Ongoing,
        }
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_p) = &self.comment_p {
            t.sub_t("comment_p", &comment_p.log());
        }
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value_p", &key_value_p.log());
        }
        t
    }
}
