//! Line syntax parser.  
//! 行構文パーサー。  

use crate::model::Element;
use crate::syntax::{machine_state::LineState, CommentP, KeyValueP, LineP, SyntaxParserResult};
use crate::token::{Token, TokenType};
use casual_logger::Table;

impl Default for LineP {
    fn default() -> Self {
        LineP {
            state: LineState::First,
            buffer: None,
            comment_p: None,
            key_value_p: None,
        }
    }
}
impl LineP {
    pub fn flush(&mut self) -> Option<Element> {
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
                            self.buffer = Some(Element::from_comment(&child_m));
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
                TokenType::Key => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = LineState::KeyValueSyntax;
                }
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = LineState::CommentSyntax;
                }
                _ => {
                    self.state = LineState::Unimplemented;
                }
            },
            LineState::KeyValueSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer = Some(Element::from_key_value(&child_m));
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
            .str("parser", "LineP#parse")
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
