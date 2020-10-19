//! Hex string parser.  
//! 16進文字列パーサー。  

use crate::model::layer110::{Token, TokenType};
use crate::parser::phase200::{
    error,
    layer210::{HexStringP, PResult},
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
    Digits,
}

impl Default for HexStringP {
    fn default() -> Self {
        HexStringP {
            buffer: Vec::new(),
            state: State::First,
            hex_number_buffer: String::new(),
            hex_number_digits: 0,
            hex_digit_count: 0,
        }
    }
}
impl HexStringP {
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
                return error(&mut self.log(), tokens, "hex_string_p.rs.66.");
            }
            State::First => {
                // TODO 汎用的に書けないか？
                // https://doc.rust-lang.org/reference/tokens.html
                self.state = State::Digits;
                self.hex_number_buffer = String::new();
                self.hex_number_digits = 4;
                self.hex_digit_count = 0;
            }
            State::Digits => match token0.type_ {
                TokenType::NumeralString
                | TokenType::AlphabetCharacter
                | TokenType::AlphabetString => {
                    let s = token0.to_string();
                    let rest = self.hex_number_digits - self.hex_digit_count;
                    let (s1, s2) = if rest < s.len() {
                        (s[0..rest].to_string(), s[rest..].to_string())
                    } else {
                        (s[0..].to_string(), "".to_string())
                    };
                    let fill = s1.len();

                    self.hex_number_buffer.push_str(&s1);
                    self.hex_digit_count += fill;

                    if self.hex_number_digits <= self.hex_digit_count {
                        let hex = u32::from_str_radix(&self.hex_number_buffer, 16).unwrap();
                        self.buffer.push(Token::new(
                            token0.column_number,
                            &from_u32(hex).unwrap().to_string(),
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }

                    if 0 < s2.len() {
                        self.buffer.push(Token::new(
                            token0.column_number,
                            &s2.to_string(),
                            TokenType::AlphabetCharacter, // TODO EscapeSequence
                        ));
                    }
                }
                _ => {
                    return error(&mut self.log(), tokens, "hex_string_p.rs.179.");
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
