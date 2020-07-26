//! Syntax parser.
//! 構文パーサー。

use crate::lexical_parser::Token;
use crate::lexical_parser::TokenType;
use crate::object_model::document::DocumentM;
use crate::object_model::line::LineM;
use crate::syntax::comment::CommentP;
use crate::syntax::key_value::KeyValueP;
use crate::syntax::SyntaxParserResult;
use casual_logger::{Log, Table};

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
            MachineState::CommentSyntax => match self.comment_p.as_mut().unwrap().parse(token) {
                SyntaxParserResult::Ok(end_of_syntax) => {
                    if end_of_syntax {
                        // ここにはこない。
                        panic!(Log::fatal_t(
                            "LineP#parse",
                            Table::default()
                                .str("parser", "LineP#parse")
                                .str("state", &format!("{:?}", self.state))
                                .str("token", &format!("{:?}", token))
                        ));
                    }
                }
                SyntaxParserResult::Err(table) => {
                    panic!(Log::fatal_t(
                        "LineP#parse",
                        Table::default()
                            .str("parser", "LineP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                            .sub_t("error", &table)
                    ));
                }
            },
            MachineState::First => match token.type_ {
                TokenType::Key => {
                    /*
                    Log::info_t(
                        "LineP#parse",
                        Table::default()
                            .str("parser", "LineP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token)),
                    );
                    */
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
                if let Some(key_value_p) = &mut self.key_value_p {
                    match key_value_p.parse(token) {
                        SyntaxParserResult::Ok(end_of_syntax) => {
                            if end_of_syntax {
                                dom.push_line(&self.product());
                                self.key_value_p = None;
                                self.state = MachineState::End;
                            }
                        } // Ignored it.
                        SyntaxParserResult::Err(table) => {
                            return SyntaxParserResult::Err(
                                Table::default()
                                    .str("parser", "LineP#parse")
                                    .str("state", &format!("{:?}", self.state))
                                    .str("token", &format!("{:?}", token))
                                    .sub_t("error", &table)
                                    .clone(),
                            );
                        }
                    }
                } else {
                    panic!(Log::fatal_t(
                        "LineP#parse",
                        Table::default()
                            .str("parser", "LineP#parse")
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    ));
                }
            }
            MachineState::Unimplemented => {
                return SyntaxParserResult::Err(
                    Table::default()
                        .str("parser", "LineP#parse")
                        .str("state", &format!("{:?}", self.state))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            MachineState::End => {
                return SyntaxParserResult::Err(
                    Table::default()
                        .str("parser", "LineP#parse")
                        .str("state", &format!("{:?}", self.state))
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
    pub fn log(&self) -> Table {
        let mut t = Table::default()
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(comment_p) = &self.comment_p {
            t.sub_t("comment", &comment_p.log());
        }
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.log());
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
