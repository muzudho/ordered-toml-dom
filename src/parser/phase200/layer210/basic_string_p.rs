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
use crate::parser::phase200::error2;
use crate::parser::phase200::error_via2;
use crate::parser::phase200::layer210::{BasicStringP, EscapeSequenceP, PResult};
use crate::parser::phase200::LookAheadTokens;
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
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            State::BeforeMultiLine => {
                // print!("trace.8.");
                self.state = State::MultiLine;
            }
            State::End => {
                return error2(&mut self.log(), tokens, "basic_string_p.rs.66.");
            }
            State::First => {
                // print!("trace.4.");
                match token0.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        // print!("trace.5.");
                        if let Some(token_1_ahead) = &tokens.one_ahead {
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
                            return error2(&mut self.log(), &tokens, "basic_string_p.rs.112.");
                        }
                    }
                    // \
                    TokenType::Backslash => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self.escape_sequence_p.as_mut().unwrap().parse(tokens) {
                            PResult::End => {
                                return error2(&mut self.log(), &tokens, "basic_string_p.rs.108.");
                            }
                            PResult::Err(mut table) => {
                                return error_via2(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
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
                        if check_triple_double_quotation(tokens.to_old()) {
                            self.state = State::MultiLineEnd1;
                        } else {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                        }
                    }
                    // \
                    TokenType::Backslash => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self.escape_sequence_p.as_mut().unwrap().parse(tokens) {
                            PResult::End => {
                                // 行末の \ だったなら。
                                // println!("[trace200 行末の \\ だったなら。]");
                                self.state = State::MultiLineTrimStart;
                            }
                            PResult::Err(mut table) => {
                                return error_via2(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
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
                        return error2(&mut self.log(), &tokens, "basic_string_p.rs.124.");
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
                        return error2(&mut self.log(), &tokens, "basic_string_p.rs.136.");
                    }
                }
            }
            State::MultiLineEscapeSequence => {
                let p = self.escape_sequence_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        self.buffer.as_mut().unwrap().extend_tokens(&p.flush());
                        self.escape_sequence_p = None;
                        self.state = State::MultiLine;
                    }
                    PResult::Err(mut table) => {
                        return error_via2(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "basic_string_p.rs.190.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
            State::MultiLineTrimStart => {
                // println!("[trace307 MultiLineTrimStart]");
                match token0.type_ {
                    TokenType::EndOfLine => {
                        // println!("[trace312 EndOfLine]");
                    } // Ignore it.
                    TokenType::WhiteSpaceString => {
                        // println!("[trace308 WhiteSpaceString]");
                    } // Ignore it.
                    // "
                    TokenType::DoubleQuotation => {
                        if check_triple_double_quotation(tokens.to_old()) {
                            // println!("[trace309 check_triple_double_quotation]");
                            self.state = State::MultiLineEnd1;
                        } else {
                            // println!("[trace310 DoubleQuotation]");
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                            self.state = State::MultiLine; // (2020-10-18追加)
                        }
                    }
                    _ => {
                        // println!("[trace311 Otherwise={:?}]", token0);
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
                    // \
                    TokenType::Backslash => {
                        self.escape_sequence_p = Some(EscapeSequenceP::default());
                        match self.escape_sequence_p.as_mut().unwrap().parse(tokens) {
                            PResult::End => {
                                return error2(&mut self.log(), &tokens, "basic_string_p.rs.252.");
                            }
                            PResult::Err(mut table) => {
                                return error_via2(
                                    &mut table,
                                    &mut self.log(),
                                    &tokens,
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
                        m.push_token(&token0);
                    }
                }
            }
            State::SingleLineEscapeSequence => {
                let p = self.escape_sequence_p.as_mut().unwrap();
                match p.parse(tokens) {
                    PResult::End => {
                        self.buffer.as_mut().unwrap().extend_tokens(&p.flush());
                        self.escape_sequence_p = None;
                        self.state = State::SingleLine;
                    }
                    PResult::Err(mut table) => {
                        return error_via2(
                            &mut table,
                            &mut self.log(),
                            &tokens,
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
