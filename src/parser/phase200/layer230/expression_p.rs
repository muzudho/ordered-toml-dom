//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::layer210::WS;
use crate::model::{layer110::TokenType, layer230::Expression};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer230::WSP;
use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable, PResult},
    layer225::KeyvalP,
    layer230::ExpressionP,
};
use casual_logger::Table;

/// Line syntax machine state.  
/// 行構文状態遷移。  
#[derive(Debug)]
pub enum State {
    AfterArrayOfTable,
    AfterKeyval,
    AfterLeftSquareBracket,
    AfterTable,
    End,
    /// `[[name]]`
    HeaderOfArrayOfTable,
    /// Whitespace and comment.
    WsComment,
    Finished,
    FirstWhitespace1,
    /// `key = val`.
    KeyvalSyntax,
    /// `[name]`
    Table,
}

impl Default for ExpressionP {
    fn default() -> Self {
        ExpressionP {
            buffer: None,
            comment_p: None,
            header_p_of_array_of_table: None,
            header_p_of_table: None,
            keyval_p: None,
            state: State::FirstWhitespace1,
            ws_p_1: None,
        }
    }
}
impl ExpressionP {
    pub fn flush(&mut self) -> Option<Expression> {
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
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            State::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), &tokens, "document_element.rs.66.");
            }
            State::AfterKeyval => match token0.type_ {
                TokenType::WhiteSpaceString => {} // Ignore it.
                // `,`
                TokenType::EndOfLine => return PResult::End,
                _ => {
                    return error(&mut self.log(), &tokens, "document_element.rs.84.");
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
                return error(&mut self.log(), &tokens, "document_element.rs.106.");
            }
            State::End => {
                return error(&mut self.log(), &tokens, "document_element.rs.98.");
            }
            State::HeaderOfArrayOfTable => {
                let p = self.header_p_of_array_of_table.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(Expression::from_header_of_array_of_table(&m));
                            self.header_p_of_array_of_table = None;
                            self.state = State::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "document_element.rs.123.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "document_element.rs.132.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::WsComment => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        self.buffer = Some(Expression::EmptyLine(
                            if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                ws_p_1.flush()
                            } else {
                                WS::default()
                            },
                            if let Some(comment_p) = self.comment_p.as_mut() {
                                comment_p.flush()
                            } else {
                                None
                            },
                        ));
                        self.ws_p_1 = None;
                        self.comment_p = None;
                        self.state = State::End;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "document_element.rs.162.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::FirstWhitespace1 => match token0.type_ {
                TokenType::EndOfLine => {
                    self.buffer = Some(Expression::EmptyLine(
                        if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                            ws_p_1.flush()
                        } else {
                            WS::default()
                        },
                        if let Some(comment_p) = self.comment_p.as_mut() {
                            comment_p.flush()
                        } else {
                            None
                        },
                    ));
                    self.ws_p_1 = None;
                    self.comment_p = None;
                    self.keyval_p = None;
                    self.state = State::Finished;
                    return PResult::End;
                }
                // `abc`
                TokenType::AbChar
                | TokenType::NumChar
                | TokenType::Hyphen
                | TokenType::Underscore => {
                    self.keyval_p = Some(KeyvalP::new());
                    match self.keyval_p.as_mut().unwrap().parse(&tokens) {
                        PResult::End => {
                            // 1トークンでは終わらないから。
                            return error(&mut self.log(), &tokens, "document_element.rs.164.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &tokens,
                                "document_element.rs.171.",
                            )
                        }
                        PResult::Ongoing => {}
                    }
                    self.state = State::KeyvalSyntax;
                }
                // `[`
                TokenType::LeftSquareBracket => {
                    self.state = State::AfterLeftSquareBracket;
                }
                // `#`
                TokenType::Sharp => {
                    self.comment_p = Some(CommentP::new());
                    self.state = State::WsComment;
                }
                TokenType::WhiteSpaceString => {
                    if let None = self.ws_p_1 {
                        self.ws_p_1 = Some(WSP::default());
                    }
                    match self.ws_p_1.as_mut().unwrap().parse(&tokens) {
                        PResult::End => {
                            return error(&mut self.log(), &tokens, "document_element.rs.197.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &tokens,
                                "document_element.rs.200.",
                            );
                        }
                        PResult::Ongoing => {}
                    }
                } // Ignored it.
                _ => {
                    return error(&mut self.log(), &tokens, "document_element.rs.246.");
                }
            },
            State::Finished => {
                return error(&mut self.log(), &tokens, "document_element.rs.205.");
            }
            State::KeyvalSyntax => {
                let p = self.keyval_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(Expression::from_keyval(&m));
                            self.keyval_p = None;
                            self.state = State::AfterKeyval;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &tokens, "document_element.rs.222.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
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
    fn parse_header_of_table(&mut self, tokens: &LookAheadTokens) -> PResult {
        let p = self.header_p_of_table.as_mut().unwrap();
        match p.parse(&tokens) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    self.buffer = Some(Expression::from_header_of_table(&m));
                    self.header_p_of_table = None;
                    self.state = State::AfterTable;
                    return PResult::End;
                } else {
                    return error(&mut self.log(), &tokens, "document_element.rs.269.");
                }
            } // Ignored it.
            PResult::Err(mut table) => {
                return error_via(
                    &mut table,
                    &mut self.log(),
                    &tokens,
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
        if let Some(p) = &self.comment_p {
            t.sub_t("comment_p", &p.log());
        }
        if let Some(p) = &self.keyval_p {
            t.sub_t("keyval_p", &p.log());
        }
        t
    }
}
