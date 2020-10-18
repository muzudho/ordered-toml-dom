//! Escape sequence parser.  
//! エスケープ・シーケンス・パーサー。  

use crate::model::layer110::{Token, TokenType};
use crate::parser::phase200::{
    error,
    layer210::{EscapeSequenceP, PResult},
};
use casual_logger::Table;
use std::char::from_u32;

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
    Unicode4digit,
    Unicode8digit,
}

impl Default for EscapeSequenceP {
    fn default() -> Self {
        EscapeSequenceP {
            buffer: Vec::new(),
            state: State::First,
        }
    }
}
impl EscapeSequenceP {
    pub fn flush(&mut self) -> Vec<Token> {
        let m = self.buffer.clone();
        self.buffer.clear();
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
                        let mut code = None;
                        match token0.to_string().as_str() {
                            "n" => code = Some("\n"),
                            "r" => code = Some("\r"),
                            "t" => code = Some("\t"),
                            "u" => {
                                self.state = State::Unicode4digit;
                            }
                            "U" => {
                                self.state = State::Unicode8digit;
                            }
                            _ => {
                                return error(&mut self.log(), tokens, "escape_sequence_p.rs.206.")
                            }
                        }
                        if let Some(code) = code {
                            self.buffer.push(Token::new(
                                token0.column_number,
                                code,
                                TokenType::AlphabetCharacter, // TODO EscapeSequence
                            ));
                            self.state = State::End;
                            return PResult::End;
                        }
                    }
                    TokenType::Backslash => {
                        self.buffer.push(Token::new(
                            token0.column_number,
                            "\\",
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    // "
                    TokenType::DoubleQuotation => {
                        self.buffer.push(Token::new(
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
            State::Unicode4digit => match token0.type_ {
                TokenType::NumeralString => {
                    let s = token0.to_string();
                    let (s1, s2) = if 4 < s.len() {
                        (s[0..4].to_string(), s[4..].to_string())
                    } else {
                        (s[0..].to_string(), "".to_string())
                    };

                    self.buffer.push(Token::new(
                        token0.column_number,
                        &from_u32(s1.parse().unwrap()).unwrap().to_string(),
                        TokenType::AlphabetCharacter, // TODO EscapeSequence
                    ));
                    if 0 < s2.len() {
                        self.buffer.push(Token::new(
                            token0.column_number + 4, // TODO だいたい4つうしろ？
                            &s2.to_string(),
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                    }
                    self.state = State::End;
                    return PResult::End;
                }
                _ => {
                    return error(&mut self.log(), tokens, "escape_sequence_p.rs.139.");
                }
            },
            State::Unicode8digit => match token0.type_ {
                TokenType::NumeralString => {
                    let s = token0.to_string();
                    let (s1, s2) = if 8 < s.len() {
                        (s[0..8].to_string(), s[8..].to_string())
                    } else {
                        (s[0..].to_string(), "".to_string())
                    };

                    self.buffer.push(Token::new(
                        token0.column_number,
                        &from_u32(s1.parse().unwrap()).unwrap().to_string(),
                        TokenType::AlphabetCharacter, // TODO EscapeSequence
                    ));
                    if 0 < s2.len() {
                        self.buffer.push(Token::new(
                            token0.column_number + 8, // TODO だいたい8つうしろ？
                            &s2.to_string(),
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                    }
                    self.state = State::End;
                    return PResult::End;
                }
                _ => {
                    return error(&mut self.log(), tokens, "escape_sequence_p.rs.139.");
                }
            },
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();

        let mut buf = String::new();
        for token in &self.buffer {
            buf.push_str(&token.to_string());
        }

        t.str("value", &buf);
        t
    }
}
