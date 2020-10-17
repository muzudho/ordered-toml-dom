//! Litera value parser.  
//! リテラル値パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralValue,
};
use crate::parser::phase200::{
    error,
    layer210::{LiteralValueP, PResult},
};
use casual_logger::Table as LogTable;

impl Default for LiteralValueP {
    fn default() -> Self {
        LiteralValueP {
            buffer: Some(LiteralValue::default()),
        }
    }
}
impl LiteralValueP {
    pub fn flush(&mut self) -> Option<LiteralValue> {
        if let Some(buffer) = &self.buffer {
            let m = Some(LiteralValue::from_str(buffer.value.trim_end())); // TODO トリム要らないのでは。
            self.buffer = None;
            return m;
        }
        None
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> PResult {
        let token0 = tokens.0.unwrap();
        match token0.type_ {
            TokenType::Alphabet
            | TokenType::Colon
            | TokenType::Dot
            | TokenType::Hyphen
            | TokenType::Numeral
            | TokenType::Plus
            | TokenType::Underscore => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);

                // Look-ahead.
                // 先読み。
                if let Some(token1) = tokens.1 {
                    match token1.type_ {
                        TokenType::Alphabet
                        | TokenType::Colon
                        | TokenType::Dot
                        | TokenType::Hyphen
                        | TokenType::Numeral
                        | TokenType::Plus
                        | TokenType::Underscore => PResult::Ongoing,
                        _ => PResult::End,
                    }
                } else {
                    PResult::End
                }
            }
            _ => return error(&mut self.log(), tokens, "literal_value_p.rs.38."),
        }
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let t = LogTable::default()
            .str("buffer", &format!("{:?}", &self.buffer))
            .clone();
        t
    }
}
