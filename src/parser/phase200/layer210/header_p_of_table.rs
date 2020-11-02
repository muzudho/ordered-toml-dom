//! Table syntax parser.  
//! テーブル構文パーサー。  

use crate::model::{
    layer110::{CharacterType, Token, TokenType},
    layer230::HeaderOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfTable, PResult};
use crate::parser::phase200::LookAheadItems<char>;
// use casual_logger::Table;

impl HeaderPOfTable {
    pub fn flush(&mut self) -> Option<HeaderOfTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        HeaderPOfTable {
            buffer: Some(HeaderOfTable::default()),
        }
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, characters: &LookAheadItems<char>) -> PResult {
        let chr0 = characters.current.as_ref().unwrap();
        match chr0.type_ {
            // `"`
            '"' => {
                // End of syntax.
                // 構文の終わり。
                return PResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&Token::from_character(&chr0, TokenType::Table));
            }
        }
        PResult::Ongoing
    }

    /* TODO
    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &m.to_string());
        }
        t
    }
    */
}
