//! Double quoted string syntax parser.  
//! 二重引用符文字列構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // "ハロー"
//!
//! // """ハロー
//! // ワールド"""
//! ```

use crate::model::{
    layer110::{Token, TokenType},
    layer210::BasicString,
};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::{BasicStringP, EscapeSequenceP, PResult};
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    BeforeMultiLine,
    End,
    // After double quotation.
    // 二重引用符の後。
    First,
    MultiLine,
    // After `\`.
    // `\` の後。
    MultiLineEnd1,
    MultiLineEnd2,
    MultiLineEscapeSequence,
    // Trim start.
    // 行頭の空白の除去。
    MultiLineTrimStart,
    SingleLine,
    // After `\`.
    // `\` の後。
    SingleLineEscapeSequence,
}

impl BasicStringP {
    pub fn flush(&mut self) -> Option<BasicString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        BasicStringP {
            escape_sequence_p: None,
            buffer: Some(BasicString::default()),
            state: State::First,
        }
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
            State::BeforeMultiLine => {
                // print!("trace.8.");
                self.state = State::MultiLine;
            }
            State::End => {
                return error(&mut self.log(), look_ahead_items, "basic_string_p.rs.66.");
            }
            State::First => {
                // print!("trace.4.");
                match chr0 {
                    // `"`
                    '"' => {
                        // print!("trace.5.");
                        if let Some(chr1_ahead) = &look_ahead_items.get(1) {
                            match chr1_ahead {
                                '"' => {
                                    //print!("trace.7.");
                                    // Before triple double quoted string.
                                    self.state = State::BeforeMultiLine;
                                }
                                _ => {
                                    // End of syntax. Empty string.
                                    // 構文の終わり。 空文字列。
                                    self.state = State::End;
                                    return PResult::End;
                                }
                            }
                        } else {
                            return error(
                                &mut self.log(),
                                &look_ahead_items,
                                "basic_string_p.rs.112.",
                            );
                        }
                    }
                    // \
                    '\\' => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self
                            .escape_sequence_p
                            .as_mut()
                            .unwrap()
                            .parse(look_ahead_items)
                        {
                            PResult::End => {
                                return error(
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "basic_string_p.rs.108.",
                                );
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "basic_string_p.rs.115.",
                                );
                            }
                            PResult::Ongoing => {
                                self.state = State::SingleLineEscapeSequence;
                            }
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr0, TokenType::BasicString));
                        self.state = State::SingleLine;
                    }
                }
            }
            State::MultiLine => {
                match chr0 {
                    // "
                    '"' => {
                        // print!("trace.10.");
                        if check_triple_double_quotation(look_ahead_items) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&Token::from_character(chr, TokenType::BasicString));
                        }
                    }
                    // \
                    '\\' => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self
                            .escape_sequence_p
                            .as_mut()
                            .unwrap()
                            .parse(look_ahead_items)
                        {
                            PResult::End => {
                                // 行末の \ だったなら。
                                // println!("[trace200 行末の \\ だったなら。]");
                                self.state = State::MultiLineTrimStart;
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "basic_string_p.rs.139.",
                                );
                            }
                            PResult::Ongoing => {
                                self.state = State::MultiLineEscapeSequence;
                            }
                        }
                    }
                    _ => {
                        // print!("trace.12.");
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr, TokenType::BasicString));
                    }
                }
            }
            State::MultiLineEnd1 => {
                match chr0 {
                    // `"`
                    '"' => {
                        self.state = State::MultiLineEnd2;
                    }
                    _ => {
                        return error(&mut self.log(), &look_ahead_items, "basic_string_p.rs.124.");
                    }
                }
            }
            State::MultiLineEnd2 => {
                match chr0 {
                    // `"`
                    '"' => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(&mut self.log(), &look_ahead_items, "basic_string_p.rs.136.");
                    }
                }
            }
            State::MultiLineEscapeSequence => {
                let p = self.escape_sequence_p.as_mut().unwrap();
                match p.parse(look_ahead_items) {
                    PResult::End => {
                        self.buffer.as_mut().unwrap().extend_tokens(&p.flush());
                        self.escape_sequence_p = None;
                        self.state = State::MultiLine;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "basic_string_p.rs.190.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::MultiLineTrimStart => {
                // println!("[trace307 MultiLineTrimStart]");
                match chr0 {
                    '\r' | '\t' => {
                        // println!("[trace312 Newline]");
                    } // Ignore it.
                    '\t' | ' ' => {
                        // println!("[trace308 WS]");
                    } // Ignore it.
                    // "
                    '"' => {
                        if check_triple_double_quotation(look_ahead_items) {
                            // println!("[trace309 check_triple_double_quotation]");
                            self.state = State::MultiLineEnd1;
                        } else {
                            // println!("[trace310 DoubleQuotation]");
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&Token::from_character(chr, TokenType::BasicString));
                            self.state = State::MultiLine; // (2020-10-18追加)
                        }
                    }
                    _ => {
                        // println!("[trace311 Otherwise={:?}]", chr0);
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr, TokenType::BasicString));
                        self.state = State::MultiLine;
                    }
                }
            }
            State::SingleLine => {
                match chr0 {
                    // `"`
                    '"' => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    // \
                    '\\' => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self
                            .escape_sequence_p
                            .as_mut()
                            .unwrap()
                            .parse(look_ahead_items)
                        {
                            PResult::End => {
                                return error(
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "basic_string_p.rs.252.",
                                );
                            }
                            PResult::Err(mut table) => {
                                return error_via(
                                    &mut table,
                                    &mut self.log(),
                                    &look_ahead_items,
                                    "basic_string_p.rs.139.",
                                );
                            }
                            PResult::Ongoing => {
                                self.state = State::SingleLineEscapeSequence;
                            }
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr, TokenType::BasicString));
                    }
                }
            }
            State::SingleLineEscapeSequence => {
                let p = self.escape_sequence_p.as_mut().unwrap();
                match p.parse(look_ahead_items) {
                    PResult::End => {
                        self.buffer.as_mut().unwrap().extend_tokens(&p.flush());
                        self.escape_sequence_p = None;
                        self.state = State::SingleLine;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &look_ahead_items,
                            "basic_string_p.rs.190.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &m.to_string());
        }
        t
    }
}

/// # Arguments
///
/// * `look_ahead_items` - Tokens contains look ahead.  
///             先読みを含むトークン。  
/// # Returns
///
/// It's triple double quotation.  
/// ３連一重引用符。  
fn check_triple_double_quotation(look_ahead_items: &LookAheadItems<char>) -> bool {
    if let Some(chr2_ahead) = &look_ahead_items.get(2) {
        match chr2_ahead {
            '"' => {
                if let Some(chr1_ahead) = &look_ahead_items.get(1) {
                    match chr1_ahead {
                        '"' => {
                            // Triple double quote.
                            true
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    } else {
        false
    }
}
