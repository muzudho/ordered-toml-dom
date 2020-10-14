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
    layer110::token::{Token, TokenType},
    layer210::DoubleQuotedString,
};
use crate::parser::phase200::{
    error,
    layer210::{DoubleQuotedStringP, PResult},
};
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    // After `\`.
    // `\` の後。
    AfterBackslash,
    DoubleQuotedString,
    End,
    // After double quotation.
    // 二重引用符の後。
    First,
    // After 2nd double quotation.
    // ２つ目の二重引用符の後。
    Second,
}

impl DoubleQuotedStringP {
    pub fn flush(&mut self) -> Option<DoubleQuotedString> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        DoubleQuotedStringP {
            buffer: Some(DoubleQuotedString::default()),
            state: State::First,
        }
    }
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_token: Option<&Token>, token: &Token) -> PResult {
        match self.state {
            State::AfterBackslash => {
                match token.type_ {
                    // `"`
                    TokenType::EndOfLine => {
                        // End of line.
                        // 行の終わり。
                        return error(&mut self.log(), token, "double_quoted_string.rs.59.");
                    }
                    _ => {
                        // Escaped.
                        self.state = State::DoubleQuotedString;
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token);
                    }
                }
            }
            State::DoubleQuotedString => {
                match token.type_ {
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
                        self.state = State::AfterBackslash;
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token);
                    }
                }
            }
            State::End => {
                return error(&mut self.log(), token, "double_quoted_string.rs.66.");
            }
            State::First => {
                match token.type_ {
                    // `"`
                    TokenType::DoubleQuotation => {
                        self.state = State::Second;
                        return PResult::End;
                    }
                    TokenType::Backslash => {
                        // Escape sequence.
                        // エスケープ・シーケンス。
                        self.state = State::AfterBackslash;
                    }
                    _ => {
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token);
                    }
                }
            }
            // After `""`.
            State::Second => {
                match token.type_ {
                    /*
                    // It's a `"""`.
                    TokenType::DoubleQuotation => {
                        self.state = State::DoubleQuotedString;
                        return PResult::End;
                    }
                    */
                    // It's a empty string. `""`.
                    TokenType::EndOfLine | TokenType::WhiteSpace => {
                        // End of syntax.
                        // 構文の終わり。
                        self.state = State::End;
                        return PResult::End;
                    }
                    // TODO `"",`.
                    _ => return error(&mut self.log(), token, "double_quoted_string.rs.130."),
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
