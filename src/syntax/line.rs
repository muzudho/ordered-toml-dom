//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::lexical_parser::TokenType;
use crate::object_model::document::DocumentM;
use crate::object_model::line::LineM;
use crate::syntax::comment::CommentP;
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
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
    pub fn product(&mut self) -> LineM {
        if let Some(p) = &self.comment_p {
            self.product.push_comment(&p.product());
        }
        if let Some(p) = &self.key_value_p {
            self.product.push_key_value(&p.product());
        }
        self.product.clone()
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
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            // ここにはこない。
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
                    SyntaxParserResult::Ok(end_of_syntax) => {
                        if end_of_syntax {
                            dom.push_line(&self.product());
                            self.key_value_p = None;
                            self.state = MachineState::End;
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
                }
            }
            MachineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            MachineState::End => {
                return SyntaxParserResult::Err(
                    self.err_table()
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
        }

        SyntaxParserResult::Ok(false)
    }
    pub fn eol(&self) -> SyntaxParserResult {
        if let Some(p) = &self.comment_p {
            p.eol()
        } else if let Some(p) = &self.key_value_p {
            p.eol()
        } else {
            SyntaxParserResult::Ok(false)
        }
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
