//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::layer210::Comment;
use crate::model::layer210::WS;
use crate::model::{layer110::CharacterType, layer230::Expression};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer230::WSP;
use crate::parser::phase200::LookAheadCharacters;
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
    AfterLeftSquareBracket,
    AfterTable,
    End,
    /// `[[name]]`
    HeaderOfArrayOfTable,
    Finished,
    /// `[name]`
    Table,
    /// Whitespace 1.
    Ws1,
    /// Whitespace 1 and comment.
    Ws1Comment,
    /// `key = val`.
    Ws1Keyval,
    Ws1KeyvalWs2,
    Ws1KeyvalWs2Comment,
}

impl Default for ExpressionP {
    fn default() -> Self {
        ExpressionP {
            buffer: None,
            comment_p: None,
            header_p_of_array_of_table: None,
            header_p_of_table: None,
            keyval_p: None,
            state: State::Ws1,
            ws_p_1: None,
            ws_p_2: None,
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
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, characters: &LookAheadCharacters) -> PResult {
        let character0 = characters.current.as_ref().unwrap();

        match self.state {
            State::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), &characters, "expression.rs.66.");
            }
            State::AfterLeftSquareBracket => match character0.type_ {
                // `[`
                CharacterType::LeftSquareBracket => {
                    self.header_p_of_array_of_table = Some(HeaderPOfArrayOfTable::new());
                    self.state = State::HeaderOfArrayOfTable;
                }
                _ => {
                    self.header_p_of_table = Some(HeaderPOfTable::new());
                    self.state = State::Table;
                    return self.parse_header_of_table(characters);
                }
            },
            State::AfterTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), &characters, "expression.rs.106.");
            }
            State::End => {
                return error(&mut self.log(), &characters, "expression.rs.98.");
            }
            State::HeaderOfArrayOfTable => {
                let p = self.header_p_of_array_of_table.as_mut().unwrap();
                match p.parse(&characters) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(Expression::from_header_of_array_of_table(&m));
                            self.header_p_of_array_of_table = None;
                            self.state = State::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &characters, "expression.rs.123.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &characters,
                            "expression.rs.132.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Finished => {
                return error(&mut self.log(), &characters, "expression.rs.205.");
            }
            State::Table => {
                return self.parse_header_of_table(characters);
            }
            State::Ws1 => match character0.type_ {
                CharacterType::Newline => {
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
                CharacterType::Alpha
                | CharacterType::Digit
                | CharacterType::Hyphen
                | CharacterType::Underscore => {
                    self.keyval_p = Some(KeyvalP::new());
                    match self.keyval_p.as_mut().unwrap().parse(&characters) {
                        PResult::End => {
                            // 1トークンでは終わらないから。
                            return error(&mut self.log(), &characters, "expression.rs.164.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &characters,
                                "expression.rs.171.",
                            )
                        }
                        PResult::Ongoing => {}
                    }
                    self.state = State::Ws1Keyval;
                }
                // `[`
                CharacterType::LeftSquareBracket => {
                    self.state = State::AfterLeftSquareBracket;
                }
                // `#`
                CharacterType::CommentStartSymbol => {
                    self.comment_p = Some(CommentP::new());
                    self.state = State::Ws1Comment;
                }
                CharacterType::Wschar => {
                    if let None = self.ws_p_1 {
                        self.ws_p_1 = Some(WSP::default());
                    }
                    match self.ws_p_1.as_mut().unwrap().parse(&characters) {
                        PResult::End => {
                            return error(&mut self.log(), &characters, "expression.rs.197.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &characters,
                                "expression.rs.200.",
                            );
                        }
                        PResult::Ongoing => {}
                    }
                } // Ignored it.
                _ => {
                    return error(&mut self.log(), &characters, "expression.rs.246.");
                }
            },
            State::Ws1Comment => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(&characters) {
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
                            &characters,
                            "expression.rs.162.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Ws1Keyval => {
                let p = self.keyval_p.as_mut().unwrap();
                match p.parse(&characters) {
                    PResult::End => {
                        let token1 = characters.one_ahead.as_ref().unwrap();

                        match token1.type_ {
                            CharacterType::Newline => {
                                if let Some(keyval) = p.flush() {
                                    self.buffer = Some(Expression::from_keyval(
                                        &if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                            ws_p_1.flush()
                                        } else {
                                            WS::default()
                                        },
                                        &keyval,
                                        &WS::default(),
                                        &Comment::default(),
                                    ));
                                    self.keyval_p = None;
                                } else {
                                    return error(
                                        &mut self.log(),
                                        &characters,
                                        "expression.rs.222.",
                                    );
                                }
                                return PResult::End;
                            }
                            CharacterType::CommentStartSymbol => {
                                self.comment_p = Some(CommentP::new());
                                self.state = State::Ws1KeyvalWs2Comment;
                            }
                            CharacterType::Wschar => {
                                self.ws_p_2 = Some(WSP::default());
                                self.state = State::Ws1KeyvalWs2;
                            }
                            _ => {
                                return error(&mut self.log(), &characters, "expression.rs.222.");
                            }
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &characters,
                            "expression.rs.231.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Ws1KeyvalWs2 => match character0.type_ {
                CharacterType::Wschar => {
                    let token1 = characters.one_ahead.as_ref().unwrap();

                    match token1.type_ {
                        CharacterType::Newline => {
                            return PResult::End;
                        }
                        CharacterType::CommentStartSymbol => {
                            self.comment_p = Some(CommentP::new());
                            self.state = State::Ws1KeyvalWs2Comment;
                        }
                        _ => {
                            return error(&mut self.log(), &characters, "expression.rs.222.");
                        }
                    }
                } // Ignore it.
                // `,`
                CharacterType::Newline => return PResult::End,
                _ => {
                    return error(&mut self.log(), &characters, "expression.rs.84.");
                }
            },
            State::Ws1KeyvalWs2Comment => {
                let p = self.comment_p.as_mut().unwrap();
                match p.parse(&characters) {
                    PResult::End => {
                        self.buffer = Some(Expression::Keyval(
                            if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                ws_p_1.flush()
                            } else {
                                WS::default()
                            },
                            self.keyval_p.as_mut().unwrap().flush().unwrap(), // TODO
                            if let Some(ws_p_2) = self.ws_p_2.as_mut() {
                                ws_p_2.flush()
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
                            &characters,
                            "expression.rs.162.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
        }

        PResult::Ongoing
    }
    /// Header of table.  
    /// テーブル・ヘッダー。  
    ///
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    fn parse_header_of_table(&mut self, characters: &LookAheadCharacters) -> PResult {
        let p = self.header_p_of_table.as_mut().unwrap();
        match p.parse(&characters) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    self.buffer = Some(Expression::from_header_of_table(&m));
                    self.header_p_of_table = None;
                    self.state = State::AfterTable;
                    return PResult::End;
                } else {
                    return error(&mut self.log(), &characters, "expression.rs.269.");
                }
            } // Ignored it.
            PResult::Err(mut table) => {
                return error_via(
                    &mut table,
                    &mut self.log(),
                    &characters,
                    "expression.rs.278.",
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
