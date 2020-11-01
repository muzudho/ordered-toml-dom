//! White space syntax parser.  
//! ホワイト・スペース構文パーサー。  

use crate::model::{
    layer110::{CharacterType, Token, TokenType},
    layer210::WS,
};
use crate::parser::phase200::layer210::{PResult, WSP};
use crate::parser::phase200::LookAheadCharacters;

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
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, characters: &LookAheadCharacters) -> PResult {
        let character0 = characters.current.as_ref().unwrap();
        match character0.type_ {
            CharacterType::Newline => return PResult::End,
            _ => {
                self.buffer
                    .push_token(&Token::from_character(character0, TokenType::WS));
            }
        }
        PResult::Ongoing
    }
}
