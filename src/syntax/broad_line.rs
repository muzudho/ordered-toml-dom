//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::BroadLine;
use crate::syntax::{
    machine_state::LineState, BroadLineP, CommentP, KeyValueP, SyntaxParserResult,
};
use crate::token::{Token, TokenType};
use casual_logger::Table;

impl Default for BroadLineP {
    fn default() -> Self {
        BroadLineP {
            buffer: None,
            comment_p: None,
            key_value_p: None,
            state: LineState::First,
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
            LineState::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(BroadLine::from_comment(&child_m));
                            self.comment_p = None;
                            self.state = LineState::AfterComment;
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
                        );
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            LineState::First => match token.type_ {
                TokenType::EndOfLine => {
                    if let Some(_) = &self.comment_p {
                        return SyntaxParserResult::End;
                    }
                    if let Some(_) = &self.key_value_p {
                        return SyntaxParserResult::End;
                    }
                    self.buffer = Some(BroadLine::EmptyLine);
                    self.state = LineState::Finished;
                    return SyntaxParserResult::End;
                }
                TokenType::Key => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = LineState::KeyValueSyntax;
                }
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = LineState::CommentSyntax;
                }
                TokenType::WhiteSpace => {} // Ignored it.
                _ => {
                    self.state = LineState::Unimplemented;
                }
            },
            LineState::Finished => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            LineState::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(BroadLine::from_key_value(&child_m));
                            self.key_value_p = None;
                            self.state = LineState::AfterKeyValue;
                            return SyntaxParserResult::End;
                        } else {
                            return SyntaxParserResult::Err(
                                self.err_table()
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    } // Ignored it.
                    SyntaxParserResult::Err(table) => {
                        return SyntaxParserResult::Err(
                            self.err_table()
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        );
                    }
                    SyntaxParserResult::Ongoing => {}
                }
            }
            LineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            LineState::AfterComment => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            LineState::AfterKeyValue => match token.type_ {
                TokenType::EndOfLine => return SyntaxParserResult::End,
                _ => {
                    return SyntaxParserResult::Err(
                        self.err_table()
                            .str("token", &format!("{:?}", token))
                            .clone(),
                    );
                }
            },
        }

        SyntaxParserResult::Ongoing
    }
    pub fn err_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "BroadLineP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_p) = &self.comment_p {
            t.sub_t("comment", &comment_p.err_table());
        }
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.err_table());
        }
        t
    }
}
