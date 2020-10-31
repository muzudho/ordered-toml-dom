//! White space syntax parser.  
//! ホワイト・スペース構文パーサー。  

use crate::model::{layer110::TokenType, layer210::WS};
use crate::parser::phase200::layer210::{PResult, WSP};
use crate::parser::phase200::LookAheadTokens;

impl Default for WSP {
    fn default() -> Self {
        WSP {
            buffer: WS::default(),
        }
    }
}
impl WSP {
    pub fn flush(&mut self) -> WS {
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
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match token0.type_ {
            TokenType::Newline => return PResult::End,
            _ => {
                self.buffer.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
}
