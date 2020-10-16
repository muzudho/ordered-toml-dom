//! Literal string syntax parser.  
//! リテラル文字列構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralValue,
};
use crate::parser::phase200::layer210::{LiteralValueP, PResult};

impl LiteralValueP {
    pub fn flush(&mut self) -> Option<LiteralValue> {
        if let Some(buffer) = &self.buffer {
            let m = Some(LiteralValue::from_str(buffer.value.trim_end()));
            self.buffer = None;
            return m;
        }
        None
    }
    pub fn new() -> Self {
        LiteralValueP {
            buffer: Some(LiteralValue::default()),
        }
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
            TokenType::EndOfLine => {
                // End of syntax.
                // 構文の終わり。
                return PResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
    /*
    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &format!("{:?}", m));
        }
        t
    }
    */
}
