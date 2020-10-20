//! Hex string parser.  
//! 16進文字列パーサー。  

use crate::model::layer110::{Token, TokenType};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{PResult, PositionalNumeralStringP};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table;

impl Default for PositionalNumeralStringP {
    fn default() -> Self {
        PositionalNumeralStringP {
            buffer: Vec::new(),
            string_buffer: String::new(),
            expected_digits: 0,
        }
    }
}
impl PositionalNumeralStringP {
    pub fn set_expected_digits(&mut self, val: usize) -> &mut Self {
        self.expected_digits = val;
        self
    }
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
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();

        match token0.type_ {
            TokenType::AbChar | TokenType::NumChar | TokenType::Underscore => {
                let s = token0.to_string();
                let expected_len = 1;
                if s.len() != expected_len {
                    // 1文字を想定しているので、そうでなかったらエラー。
                    panic!("s.len()={} s=|{}|", s.len(), s);
                }

                self.string_buffer.push_str(&s);

                // 次がHexの文字以外か？
                let finished = if let Some(token1) = tokens.one_ahead.as_ref() {
                    match token1.type_ {
                        TokenType::AbChar | TokenType::NumChar | TokenType::Underscore => {
                            // 続行。
                            false
                        }
                        _ => true,
                    }
                } else {
                    true
                };

                // Filled.
                // 満ちたなら。
                if finished
                    || (self.expected_digits != 0
                        && self.expected_digits <= self.string_buffer.len())
                {
                    /*
                    println!(
                        // "[trace56={} self.expected_digits={} self.string_buffer.len()={} tokens.one_ahead={}]",
                        self.string_buffer,
                        self.expected_digits,
                        self.string_buffer.len(),
                        if let Some(token1) = tokens.one_ahead.as_ref() {token1.to_string()}else{"".to_string()}
                    );
                    // */
                    self.buffer.push(Token::new(
                        token0.column_number,
                        &self.string_buffer,
                        TokenType::SPPositionalNumeralString,
                    ));
                    return PResult::End;
                }

                // １文字ずつだから、オーバーフローしないはず。
            }
            _ => {
                return error(&mut self.log(), &tokens, "hex_string_p.rs.179.");
            }
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
