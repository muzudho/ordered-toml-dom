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
use crate::parser::phase200::{
    error,
    layer210::{BasicStringP, PResult},
};
use casual_logger::Table;

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
    MultiLineAfterBackslashNotEscaped,
    MultiLineAfterBackslashEscaped,
    MultiLineEnd1,
    MultiLineEnd2,
    // Trim start.
    // 行頭の空白の除去。
    MultiLineTrimStart,
    // After `\`.
    // `\` の後。
    SingleLineAfterBackslash,
    SingleLine,
}

impl BasicStringP {
    pub fn flush(&mut self) -> Option<BasicString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        BasicStringP {
            buffer: Some(BasicString::default()),
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
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            State::BeforeMultiLine => {
                // print!("trace.8.");
                self.state = State::MultiLine;
            }
            State::End => {
                return error(&mut self.log(), tokens, "basic_string_p.rs.66.");
            }
            State::First => {
                // print!("trace.4.");
                match token0.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        // print!("trace.5.");
                        if let Some(token_1_ahead) = tokens.1 {
                            match token_1_ahead.type_ {
                                TokenType::DoubleQuotation => {
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
                            return error(&mut self.log(), tokens, "basic_string_p.rs.112.");
                        }
                    }
                    TokenType::Backslash => {
                        // print!("trace.6.");
                        // Escape sequence.
                        // エスケープ・シーケンス。
                        self.state = State::SingleLineAfterBackslash;
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
                    // "
                    TokenType::DoubleQuotation => {
                        // print!("trace.10.");
                        if check_triple_double_quotation(tokens) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                        }
                    }
                    // \
                    TokenType::Backslash => {
                        // Escape sequence.
                        // エスケープ・シーケンス。
                        if let Some(token_1_ahead) = tokens.1 {
                            match token_1_ahead.type_ {
                                TokenType::AlphabetCharacter => {
                                    // print!("[trace1 ahead={:?}]", token_1_ahead);
                                    // Backslash.
                                    let m = self.buffer.as_mut().unwrap();
                                    m.push_token(&token0);
                                    self.state = State::MultiLineAfterBackslashEscaped;
                                }
                                TokenType::Backslash => {
                                    // print!("[trace2 (IgnoreBackslash) ahead={:?}]", token_1_ahead);
                                    self.state = State::MultiLineAfterBackslashEscaped;
                                }
                                TokenType::EndOfLine => {
                                    // print!("[trace3 EndOfLIne]");
                                    self.state = State::MultiLineAfterBackslashNotEscaped;
                                }
                                _ => {
                                    return error(
                                        &mut self.log(),
                                        tokens,
                                        "basic_string_p.rs.136.",
                                    );
                                }
                            }
                        } else {
                            return error(&mut self.log(), tokens, "basic_string_p.rs.112.");
                        }
                    }
                    _ => {
                        // print!("trace.12.");
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                    }
                }
            }
            State::MultiLineEnd1 => {
                match token0.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        self.state = State::MultiLineEnd2;
                    }
                    _ => {
                        return error(&mut self.log(), tokens, "basic_string_p.rs.124.");
                    }
                }
            }
            State::MultiLineEnd2 => {
                match token0.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(&mut self.log(), tokens, "basic_string_p.rs.136.");
                    }
                }
            }
            State::MultiLineAfterBackslashEscaped => {
                // println!("[trace196 token0=|{:?}|]", token0);
                // Escaped.
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
                self.state = State::MultiLine;
            }
            State::MultiLineAfterBackslashNotEscaped => {
                self.state = State::MultiLineTrimStart;
            }
            State::MultiLineTrimStart => {
                match token0.type_ {
                    TokenType::WhiteSpaceString => {} // Ignore it.
                    // "
                    TokenType::DoubleQuotation => {
                        if check_triple_double_quotation(tokens) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                        }
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                        self.state = State::MultiLine;
                    }
                }
            }
            State::SingleLine => {
                match token0.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    TokenType::Backslash => {
                        // Escape sequence.
                        // エスケープ・シーケンス。
                        self.state = State::SingleLineAfterBackslash;
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token0);
                    }
                }
            }
            State::SingleLineAfterBackslash => {
                match token0.type_ {
                    // `"`
                    TokenType::EndOfLine => {
                        // End of line.
                        // 行の終わり。
                        return error(&mut self.log(), tokens, "basic_string_p.rs.59.");
                    }
                    _ => {
                        // Escaped.
                        self.state = State::SingleLine;
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
            t.str("value", &format!("{:?}", m));
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
/// It's triple double quotation.  
/// ３連一重引用符。  
fn check_triple_double_quotation(tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> bool {
    if let Some(token_2_ahead) = tokens.2 {
        match token_2_ahead.type_ {
            TokenType::DoubleQuotation => {
                if let Some(token_1_ahead) = tokens.1 {
                    match token_1_ahead.type_ {
                        TokenType::DoubleQuotation => {
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
