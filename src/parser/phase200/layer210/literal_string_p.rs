//! Single quoted string syntax parser.  
//! 単一引用符文字列構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralString,
};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{LiteralStringP, PResult};
use casual_logger::Table;
use look_ahead_items::LookAheadItems;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    BeforeMultiLine1,
    BeforeMultiLine2,
    End,
    // After double quotation.
    // 二重引用符の後。
    First,
    MultiLine,
    MultiLineEnd1,
    MultiLineEnd2,
    SingleLine,
}

impl LiteralStringP {
    pub fn flush(&mut self) -> Option<LiteralString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        LiteralStringP {
            buffer: Some(LiteralString::default()),
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
            State::BeforeMultiLine1 => {
                // Skip 3rd single quotation.
                // Look-ahead.
                // 先読み。
                if let Some(chr1_ahead) = &look_ahead_items.get(1) {
                    match chr1_ahead {
                        '\r' | '\t' => {
                            self.state = State::BeforeMultiLine2;
                        }
                        _ => {
                            self.state = State::MultiLine;
                        }
                    }
                } else {
                    return error(
                        &mut self.log(),
                        &look_ahead_items,
                        "literal_string_p.rs.67.",
                    );
                }
            }
            State::BeforeMultiLine2 => {
                // Skip first end-of-line.
                self.state = State::MultiLine;
            }
            State::End => {
                return error(
                    &mut self.log(),
                    &look_ahead_items,
                    "literal_string_p.rs.66.",
                );
            }
            State::First => {
                match chr0 {
                    // `'`
                    '\'' => {
                        // Look-ahead.
                        // 先読み。
                        if let Some(chr1_ahead) = &look_ahead_items.get(1) {
                            match chr1_ahead {
                                '\'' => {
                                    // Before triple sinble quoted string.
                                    self.state = State::BeforeMultiLine1;
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
                                "literal_string_p.rs.112.",
                            );
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr0, TokenType::LiteralString));
                        self.state = State::SingleLine;
                    }
                }
            }
            State::MultiLine => {
                match chr0.type_ {
                    // `'`
                    '\'' => {
                        if check_triple_single_quotation(look_ahead_items) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&Token::from_character(chr0, TokenType::LiteralString));
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr0, TokenType::LiteralString));
                    }
                }
            }
            State::MultiLineEnd1 => {
                match chr0 {
                    // `'`
                    '\'' => {
                        self.state = State::MultiLineEnd2;
                    }
                    _ => {
                        return error(
                            &mut self.log(),
                            &look_ahead_items,
                            "literal_string_p.rs.124.",
                        );
                    }
                }
            }
            State::MultiLineEnd2 => {
                match chr0 {
                    // `'`
                    '\'' => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(
                            &mut self.log(),
                            &look_ahead_items,
                            "literal_string_p.rs.136.",
                        );
                    }
                }
            }
            State::SingleLine => {
                match chr0 {
                    // `'`
                    '\'' => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::from_character(chr, TokenType::LiteralString));
                    }
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
/// It's triple single quotation.  
/// ３連一重引用符。  
fn check_triple_single_quotation(look_ahead_items: &LookAheadItems<char>) -> bool {
    if let Some(chr2_ahead) = &look_ahead_items.get(2) {
        match chr2_ahead {
            '\'' => {
                if let Some(chr1_ahead) = &look_ahead_items.get(1) {
                    match chr1_ahead {
                        '\'' => {
                            // Triple single quote.
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
