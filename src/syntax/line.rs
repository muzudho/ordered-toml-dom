//! Syntax parser.
//! 構文パーサー。

use crate::object_model::document::DocumentM;
use crate::object_model::line::LineM;
use crate::syntax::comment::CommentP;
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
use crate::token::{Token, TokenType};
use casual_logger::Table;

pub struct LineP {
    state: MachineState,
    product: LineM,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
}
impl Default for LineP {
    fn default() -> Self {
        LineP {
            state: MachineState::First,
            product: LineM::default(),
            comment_p: None,
            key_value_p: None,
        }
    }
}
impl LineP {
    pub fn product(&mut self) -> &LineM {
        if let Some(p) = &self.comment_p {
            self.product.push_comment(&p.product());
        }
        if let Some(p) = &self.key_value_p {
            self.product.push_key_value(&p.product());
        }
        &self.product
    }

    /// # Returns
    ///
    /// * `SyntaxParserResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token, dom: &mut DocumentM) -> SyntaxParserResult {
        match self.state {
            MachineState::CommentSyntax => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        dom.push_line(&self.product());
                        self.state = MachineState::End;
                        return SyntaxParserResult::End;
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
            MachineState::First => match token.type_ {
                TokenType::Key => {
                    self.key_value_p = Some(KeyValueP::new(&token));
                    self.state = MachineState::KeyPairSyntax;
                }
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = MachineState::CommentSyntax;
                }
                _ => {
                    self.state = MachineState::Unimplemented;
                }
            },
            MachineState::KeyPairSyntax => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    SyntaxParserResult::End => {
                        dom.push_line(&self.product());
                        self.key_value_p = None;
                        self.state = MachineState::End;
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
            MachineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            MachineState::End => match token.type_ {
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

#[derive(Debug)]
enum MachineState {
    /// `# comment`.
    CommentSyntax,
    End,
    First,
    /// `key = right_value`.
    KeyPairSyntax,
    Unimplemented,
}
