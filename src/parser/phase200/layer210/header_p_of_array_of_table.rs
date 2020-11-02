//! Array of HeaderOfArrayOfTable syntax parser.  
//! テーブルの配列構文パーサー。  

use crate::model::{
    layer110::{CharacterType, TokenType},
    layer230::HeaderOfArrayOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfArrayOfTable, PResult};
use crate::parser::phase200::LookAheadItems<char>;
use crate::parser::phase200::Token;
// use casual_logger::Table;

impl HeaderPOfArrayOfTable {
    pub fn flush(&mut self) -> Option<HeaderOfArrayOfTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        HeaderPOfArrayOfTable {
            buffer: Some(HeaderOfArrayOfTable::default()),
        }
    }
    /// # Arguments
    ///
    /// * `look_ahead_items` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, look_ahead_items: &LookAheadItems<char>) -> PResult {
        let chr0 = look_ahead_items.get(0).unwrap();
        match chr0 {
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
