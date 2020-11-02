//! Broad-line syntax parser.  
//! `縦幅のある行` 構文パーサー。  

use crate::model::layer210::Comment;
use crate::model::layer210::Ws;
use crate::model::{layer110::CharacterType, layer230::Expression};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer230::WsP;
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable, PResult},
    layer225::KeyvalP,
    layer230::ExpressionP,
};
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

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
    First,
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
            state: State::First,
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
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();

        match self.state {
            State::AfterArrayOfTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), &look_ahead_items, "expression.rs.66.");
            }
            State::AfterLeftSquareBracket => match chr0 {
                '[' => {
                    self.header_p_of_array_of_table = Some(HeaderPOfArrayOfTable::new());
                    self.state = State::HeaderOfArrayOfTable;
                }
                _ => {
                    self.header_p_of_table = Some(HeaderPOfTable::new());
                    self.state = State::Table;
                    return self.parse_header_of_table(look_ahead_items);
                }
            },
            State::AfterTable => {
                // TODO 後ろにコメントがあるかも。
                return error(&mut self.log(), &look_ahead_items, "expression.rs.106.");
            }
            State::End => {
                return error(&mut self.log(), &look_ahead_items, "expression.rs.98.");
            }
            State::HeaderOfArrayOfTable => {
                let p = self.header_p_of_array_of_table.as_mut().unwrap();
                match p.parse(&look_ahead_items) {
                    PResult::End => {
                        if let Some(m) = p.flush() {
                            self.buffer = Some(Expression::from_header_of_array_of_table(&m));
                            self.header_p_of_array_of_table = None;
                            self.state = State::AfterArrayOfTable;
                            return PResult::End;
                        } else {
                            return error(&mut self.log(), &look_ahead_items, "expression.rs.123.");
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "expression.rs.132.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Finished => {
                return error(&mut self.log(), &look_ahead_items, "expression.rs.205.");
            }
            State::Table => {
                return self.parse_header_of_table(look_ahead_items);
            }
            State::First | State::Ws1 => match chr0 {
                '\r' | '\n' => {
                    self.buffer = Some(Expression::EmptyLine(
                        if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                            ws_p_1.get_ws()
                        } else {
                            Ws::default()
                        },
                        if let Some(comment_p) = self.comment_p.as_mut() {
                            Some(comment_p.get_product())
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
                CharacterType::Alpha | CharacterType::Digit | '-' | '_' => {
                    self.keyval_p = Some(KeyvalP::new());
                    match self.keyval_p.as_mut().unwrap().parse(&look_ahead_items) {
                        PResult::End => {
                            // 1トークンでは終わらないから。
                            return error(&mut self.log(), &look_ahead_items, "expression.rs.164.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &look_ahead_items,
                                "expression.rs.171.",
                            )
                        }
                        PResult::Ongoing => {}
                    }
                    self.state = State::Ws1Keyval;
                }
                // `[`
                '[' => {
                    self.state = State::AfterLeftSquareBracket;
                }
                // `#`
                '#' => {
                    self.comment_p = Some(CommentP::new());

                    let p = self.comment_p.as_mut().unwrap();
                    let judge = p.judge1(&chr0);
                    if let Some(judge) = judge {
                        p.commit1(&judge);
                        match p.forward1(&look_ahead_items) {
                            PResult::End => {
                                self.buffer = Some(Expression::EmptyLine(
                                    Ws::default(),
                                    if let Some(comment_p) = self.comment_p.as_mut() {
                                        Some(comment_p.get_product())
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
                                    &look_ahead_items,
                                    "expression.rs.162.",
                                );
                            }
                            PResult::Ongoing => {
                                self.state = State::Ws1Comment;
                            }
                        }
                    } else {
                        return error(&mut self.log(), &look_ahead_items, "expression.rs.236.");
                    }
                }
                '\t' | ' ' => {
                    if let None = self.ws_p_1 {
                        self.ws_p_1 = Some(WsP::default());
                    }
                    match self.ws_p_1.as_mut().unwrap().parse(&look_ahead_items) {
                        PResult::End => {
                            return error(&mut self.log(), &look_ahead_items, "expression.rs.197.");
                        }
                        PResult::Err(mut table) => {
                            return error_via(
                                &mut table,
                                &mut self.log(),
                                &look_ahead_items,
                                "expression.rs.200.",
                            );
                        }
                        PResult::Ongoing => {}
                    }
                } // Ignored it.
                _ => {
                    return error(&mut self.log(), &look_ahead_items, "expression.rs.246.");
                }
            },
            State::Ws1Comment => {
                let p = self.comment_p.as_mut().unwrap();
                let judge = p.judge1(&chr0);
                if let Some(judge) = judge {
                    p.commit1(&judge);
                    match p.forward1(&look_ahead_items) {
                        PResult::End => {
                            self.buffer = Some(Expression::EmptyLine(
                                if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                    ws_p_1.get_ws()
                                } else {
                                    Ws::default()
                                },
                                if let Some(comment_p) = self.comment_p.as_mut() {
                                    Some(comment_p.get_product())
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
                                &look_ahead_items,
                                "expression.rs.162.",
                            );
                        }
                        PResult::Ongoing => {}
                    }
                } else {
                    return error(&mut self.log(), &look_ahead_items, "expression.rs.236.");
                }
            }
            State::Ws1Keyval => {
                let p = self.keyval_p.as_mut().unwrap();
                match p.parse(&look_ahead_items) {
                    PResult::End => {
                        let token1 = look_ahead_items.one_ahead.as_ref().unwrap();

                        match token1.type_ {
                            '\r' | '\t' => {
                                if let Some(keyval) = p.flush() {
                                    self.buffer = Some(Expression::from_keyval(
                                        &if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                            ws_p_1.get_ws()
                                        } else {
                                            Ws::default()
                                        },
                                        &keyval,
                                        &Ws::default(),
                                        &Comment::default(),
                                    ));
                                    self.keyval_p = None;
                                } else {
                                    return error(
                                        &mut self.log(),
                                        &look_ahead_items,
                                        "expression.rs.222.",
                                    );
                                }
                                return PResult::End;
                            }
                            '#' => {
                                self.comment_p = Some(CommentP::new());
                                self.state = State::Ws1KeyvalWs2Comment;
                            }
                            '\t' | ' ' => {
                                self.ws_p_2 = Some(WsP::default());
                                self.state = State::Ws1KeyvalWs2;
                            }
                            _ => {
                                return error(
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "expression.rs.222.",
                                );
                            }
                        }
                    } // Ignored it.
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "expression.rs.231.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::Ws1KeyvalWs2 => match chr0 {
                '\t' | ' ' => {
                    let token1 = look_ahead_items.one_ahead.as_ref().unwrap();

                    match token1.type_ {
                        '\r' | '\t' => {
                            return PResult::End;
                        }
                        '#' => {
                            self.comment_p = Some(CommentP::new());
                            self.state = State::Ws1KeyvalWs2Comment;
                        }
                        _ => {
                            return error(&mut self.log(), &look_ahead_items, "expression.rs.222.");
                        }
                    }
                } // Ignore it.
                // `,`
                '\r' | '\t' => return PResult::End,
                _ => {
                    return error(&mut self.log(), &look_ahead_items, "expression.rs.84.");
                }
            },
            State::Ws1KeyvalWs2Comment => {
                let p = self.comment_p.as_mut().unwrap();
                let judge = p.judge1(&chr0);
                if let Some(judge) = judge {
                    p.commit1(&judge);
                    match p.forward1(&look_ahead_items) {
                        PResult::End => {
                            self.buffer = Some(Expression::Keyval(
                                if let Some(ws_p_1) = self.ws_p_1.as_mut() {
                                    ws_p_1.get_ws()
                                } else {
                                    Ws::default()
                                },
                                self.keyval_p.as_mut().unwrap().flush().unwrap(), // TODO
                                if let Some(ws_p_2) = self.ws_p_2.as_mut() {
                                    ws_p_2.get_ws()
                                } else {
                                    Ws::default()
                                },
                                if let Some(comment_p) = self.comment_p.as_mut() {
                                    Some(comment_p.get_product())
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
                                &look_ahead_items,
                                "expression.rs.162.",
                            );
                        }
                        PResult::Ongoing => {}
                    }
                } else {
                    return error(&mut self.log(), &look_ahead_items, "expression.rs.315.");
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
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    fn parse_header_of_table(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let p = self.header_p_of_table.as_mut().unwrap();
        match p.parse(&look_ahead_items) {
            PResult::End => {
                if let Some(m) = p.flush() {
                    self.buffer = Some(Expression::from_header_of_table(&m));
                    self.header_p_of_table = None;
                    self.state = State::AfterTable;
                    return PResult::End;
                } else {
                    return error(&mut self.log(), &look_ahead_items, "expression.rs.269.");
                }
            } // Ignored it.
            PResult::Err(mut table) => {
                return error_via(
                    &mut table,
                    &mut self.log(),
                    &look_ahead_items,
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
