//! Single quoted string syntax parser.  
//! 単一引用符文字列構文パーサー。  

use crate::model::{layer110::TokenType, layer210::LiteralString};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{LiteralStringP, PResult};
use crate::parser::phase200::LookAheadCharacters;
use casual_logger::Table;

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
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, tokens: &LookAheadCharacters) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            State::BeforeMultiLine1 => {
                // Skip 3rd single quotation.
                // Look-ahead.
                // 先読み。
                if let Some(token_1_ahead) = &tokens.one_ahead {
                    match token_1_ahead.type_ {
                        TokenType::Newline => {
                            self.state = State::BeforeMultiLine2;
                        }
                        _ => {
                            self.state = State::MultiLine;
                        }
                    }
                } else {
                    return error(&mut self.log(), &tokens, "literal_string_p.rs.67.");
                }
            }
            State::BeforeMultiLine2 => {
                // Skip first end-of-line.
                self.state = State::MultiLine;
            }
            State::End => {
                return error(&mut self.log(), &tokens, "literal_string_p.rs.66.");
            }
            State::First => {
                match token0.type_ {
                    // `'`
                    TokenType::SingleQuotation => {
                        // Look-ahead.
                        // 先読み。
                        if let Some(token_1_ahead) = &tokens.one_ahead {
                            match token_1_ahead.type_ {
                                TokenType::SingleQuotation => {
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
                            return error(&mut self.log(), &tokens, "literal_string_p.rs.112.");
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                        self.state = State::SingleLine;
                    }
                }
            }
            State::MultiLine => {
                match token0.type_ {
                    // `'`
                    TokenType::SingleQuotation => {
                        if check_triple_single_quotation(tokens) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                    }
                }
            }
            State::MultiLineEnd1 => {
                match token0.type_ {
                    // `'`
                    TokenType::SingleQuotation => {
                        self.state = State::MultiLineEnd2;
                    }
                    _ => {
                        return error(&mut self.log(), &tokens, "literal_string_p.rs.124.");
                    }
                }
            }
            State::MultiLineEnd2 => {
                match token0.type_ {
                    // `'`
                    TokenType::SingleQuotation => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(&mut self.log(), &tokens, "literal_string_p.rs.136.");
                    }
                }
            }
            State::SingleLine => {
                match token0.type_ {
                    // `'`
                    TokenType::SingleQuotation => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
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
/// * `tokens` - Tokens contains look ahead.  
///             先読みを含むトークン。  
/// # Returns
///
/// It's triple single quotation.  
/// ３連一重引用符。  
fn check_triple_single_quotation(tokens: &LookAheadCharacters) -> bool {
    if let Some(token_2_ahead) = &tokens.two_ahead {
        match token_2_ahead.type_ {
            TokenType::SingleQuotation => {
                if let Some(token_1_ahead) = &tokens.one_ahead {
                    match token_1_ahead.type_ {
                        TokenType::SingleQuotation => {
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
