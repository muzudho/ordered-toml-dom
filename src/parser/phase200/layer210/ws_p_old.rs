//! White space syntax parser.  
//! ホワイト・スペース構文パーサー。  

use crate::model::{
    layer110::{CharacterType, Token, TokenType},
    layer210::WSOld,
};
use crate::parser::phase200::layer210::{PResult, WSPOld};
use crate::parser::phase200::LookAheadCharacters;

impl Default for WSPOld {
    fn default() -> Self {
        WSPOld {
            buffer: WSOld::default(),
        }
    }
}
impl WSPOld {
    pub fn flush(&mut self) -> WSOld {
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
                    .push_token(&Token::from_character(character0, TokenType::WSOld));
            }
        }
        PResult::Ongoing
    }
}
