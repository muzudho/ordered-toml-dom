//! Inline table syntax parser.  
//! インライン・テーブル構文パーサー。  

use crate::model::{
    layer20::InlineTable,
    layer5::token::{Token, TokenType},
};
use crate::parser::syntax::{
    layer10::PResult,
    layer20::{usize_to_i128, InlineTableP, KeyValueP},
};
use casual_logger::{Log, Table};

/// Inline table syntax machine state.  
/// インライン・テーブル構文状態遷移。  
///
/// Example: `{ key = value, key = value }`.  
#[derive(Debug)]
pub enum InlineTableState {
    AfterLeftCurlyBracket,
    KeyValue,
    AfterKeyValue,
}

impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: InlineTableState::AfterLeftCurlyBracket,
            buffer: Some(InlineTable::default()),
            key_value_p: None,
        }
    }
}
impl InlineTableP {
    pub fn flush(&mut self) -> Option<InlineTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, token: &Token) -> PResult {
        match self.state {
            InlineTableState::AfterLeftCurlyBracket => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    TokenType::Key => {
                        self.key_value_p = Some(Box::new(KeyValueP::new(&token)));
                        self.state = InlineTableState::KeyValue;
                    }
                    _ => panic!(Log::fatal_t(
                        "InlineTableP#parse/AfterValue",
                        self.log_table()
                            .int("column_number", usize_to_i128(token.column_number))
                            .str("state", &format!("{:?}", self.state))
                            .str("token", &format!("{:?}", token))
                    )),
                }
            }
            InlineTableState::KeyValue => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_key_value(&child_m);
                            self.key_value_p = None;
                            self.state = InlineTableState::AfterKeyValue;
                        } else {
                            return PResult::Err(
                                self.log_table()
                                    .int("column_number", usize_to_i128(token.column_number))
                                    .str("token", &format!("{:?}", token))
                                    .clone(),
                            );
                        }
                    }
                    PResult::Err(table) => {
                        return PResult::Err(
                            self.log_table()
                                .int("column_number", usize_to_i128(token.column_number))
                                .str("token", &format!("{:?}", token))
                                .sub_t("error", &table)
                                .clone(),
                        )
                    }
                    PResult::Ongoing => {}
                }
            }
            InlineTableState::AfterKeyValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                // `,`
                TokenType::Comma => {
                    self.state = InlineTableState::AfterLeftCurlyBracket;
                }
                // `}`
                TokenType::RightCurlyBracket => {
                    return PResult::End;
                }
                _ => panic!(Log::fatal_t(
                    "InlineTableP#parse/AfterValue",
                    self.log_table()
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token))
                )),
            },
        }
        PResult::Ongoing
    }
    pub fn log_table(&self) -> Table {
        let mut t = Table::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.log_table());
        }
        t
    }
}
