//! Double quoted string syntax parser.  
//! 二重引用符文字列構文パーサー。  
//!
//! # Examples
//!
//! ```
//! // "ハロー"
//! ```

use crate::model::{
    layer110::token::{Token, TokenType},
    layer210::DoubleQuotedString,
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, PResult},
    layer220::usize_to_i128,
};
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    // After double quotation.
    // 二重引用符の後。
    First,
    // After `\`.
    // `\` の後。
    AfterBackslash,
    End,
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
    pub fn parse(&mut self, token: &Token) -> PResult {
        match self.state {
            State::AfterBackslash => {
                match token.type_ {
                    // `"`
                    TokenType::EndOfLine => {
                        // End of line.
                        // 行の終わり。
                        return PResult::Err(
                            self.log_snapshot()
                                .str("place_of_occurrence", "double_quoted_string.rs.59.")
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .clone(),
                        );
                    }
                    _ => {
                        // Escaped.
                        self.state = State::First;
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&token);
                    }
                }
            }
            State::End => {
                return PResult::Err(
                    self.log_snapshot()
                        .str("place_of_occurrence", "double_quoted_string.rs.66.")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                        .clone(),
                );
            }
            State::First => {
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
        }

        PResult::Ongoing
    }
    pub fn log_snapshot(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
}
