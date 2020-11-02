//! Hex string parser.  
//! 16進文字列パーサー。  

use crate::model::layer110::{CharacterType, Token, TokenType};
use crate::parser::phase200::error;
use crate::parser::phase200::layer210::{PResult, PositionalNumeralStringP};
use casual_logger::Table;

impl PositionalNumeralStringP {
    pub fn new(prefix: &str) -> Self {
        PositionalNumeralStringP {
            buffer: Vec::new(),
            prefix: prefix.to_string(),
            string_buffer: String::new(),
            expected_digits: 0,
        }
    }
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
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let token0 = look_ahead_items.get(0).unwrap();

        match token0.type_ {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
                let s = token0.to_string();
                let expected_len = 1;
                if s.len() != expected_len {
                    // 1文字を想定しているので、そうでなかったらエラー。
                    panic!("s.len()={} s=|{}|", s.len(), s);
                }

                self.string_buffer.push_str(&s);

                // 次がHexの文字以外か？
                let finished = if let Some(token1) = look_ahead_items.one_ahead.as_ref() {
                    match token1.type_ {
                        'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {
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
                        // "[trace56={} self.expected_digits={} self.string_buffer.len()={} look_ahead_items.one_ahead={}]",
                        self.string_buffer,
                        self.expected_digits,
                        self.string_buffer.len(),
                        if let Some(token1) = look_ahead_items.one_ahead.as_ref() {token1.to_string()}else{"".to_string()}
                    );
                    // */
                    self.buffer.push(Token::new(
                        &self.string_buffer,
                        TokenType::SPPositionalNumeralString,
                    ));
                    return PResult::End;
                }

                // １文字ずつだから、オーバーフローしないはず。
            }
            _ => {
                return error(&mut self.log(), &look_ahead_items, "hex_string_p.rs.179.");
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
