//! Escape sequence parser.  
//! エスケープ・シーケンス・パーサー。  

use crate::model::layer110::{Token, TokenType};
use crate::parser::phase200::{
    error,
    layer210::{EscapeSequenceP, PResult},
};
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    // After double quotation.
    // 二重引用符の後。
    First,
    // After `\`.
    // `\` の後。
    EscapedCharacter,
}

impl Default for EscapeSequenceP {
    fn default() -> Self {
        EscapeSequenceP {
            buffer: None,
            state: State::First,
        }
    }
}
impl EscapeSequenceP {
    pub fn flush(&mut self) -> Option<Token> {
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
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match self.state {
            State::End => {
                return error(&mut self.log(), tokens, "escape_sequence_p.rs.66.");
            }
            State::First => {
                // Look-ahead.
                // 先読み。
                if let Some(token_1_ahead) = tokens.1 {
                    match token_1_ahead.type_ {
                        TokenType::AlphabetCharacter
                        | TokenType::Backslash
                        | TokenType::DoubleQuotation => {
                            // print!("[trace1 (IgnoreBackslash) ahead={:?}]", token_1_ahead);
                            self.state = State::EscapedCharacter;
                        }
                        TokenType::EndOfLine => {
                            // 行末に \ があったケース。
                            // println!("[trace3 行末にEOLがあったケース]");
                            self.state = State::End;
                            return PResult::End;
                        }
                        _ => {
                            return error(&mut self.log(), tokens, "escape_sequence_p.rs.136.");
                        }
                    }
                } else {
                    return error(&mut self.log(), tokens, "escape_sequence_p.rs.112.");
                }
            }
            State::EscapedCharacter => {
                // println!("[trace196={:?}]", token0);
                // Escaped.
                match token0.type_ {
                    // `"`
                    TokenType::AlphabetCharacter => {
                        // TODO 汎用的に書けないか？
                        // https://doc.rust-lang.org/reference/tokens.html
                        let code = match token0.to_string().as_str() {
                            "n" => "\n",
                            "r" => "\r",
                            "t" => "\t",
                            _ => {
                                return error(&mut self.log(), tokens, "escape_sequence_p.rs.206.")
                            }
                        };
                        self.buffer = Some(Token::new(
                            token0.column_number,
                            code,
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    TokenType::Backslash => {
                        self.buffer = Some(Token::new(
                            token0.column_number,
                            "\\",
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    // "
                    TokenType::DoubleQuotation => {
                        self.buffer = Some(Token::new(
                            token0.column_number,
                            "\"",
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(&mut self.log(), tokens, "escape_sequence_p.rs.212.");
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
            t.str("value", &format!("{}", m));
        }
        t
    }
}
