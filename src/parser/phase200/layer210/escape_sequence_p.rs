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
                            // print!("[trace3 EndOfLIne]");
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
                        match token0.to_string().as_str() {
                            "n" => {
                                self.buffer = Some(Token::new(
                                    token0.column_number,
                                    "\n",
                                    TokenType::AlphabetCharacter, // TODO EscapeSequence
                                ));
                            }
                            "r" => {
                                self.buffer = Some(Token::new(
                                    token0.column_number,
                                    "\r",
                                    TokenType::AlphabetCharacter, // TODO EscapeSequence
                                ));
                            }
                            "t" => {
                                self.buffer = Some(Token::new(
                                    token0.column_number,
                                    "\t",
                                    TokenType::AlphabetCharacter, // TODO EscapeSequence
                                ));
                            }
                            _ => {
                                return error(&mut self.log(), tokens, "escape_sequence_p.rs.206.");
                            }
                        }
                    }
                    TokenType::Backslash => {
                        self.buffer = Some(Token::new(
                            token0.column_number,
                            "\\",
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                    }
                    // "
                    TokenType::DoubleQuotation => {
                        self.buffer = Some(Token::new(
                            token0.column_number,
                            "\"",
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                    }
                    _ => {
                        return error(&mut self.log(), tokens, "escape_sequence_p.rs.212.");
                    }
                }
                self.state = State::First;
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
