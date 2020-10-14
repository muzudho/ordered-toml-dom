//! Inline table syntax parser.  
//! インライン・テーブル構文パーサー。  

use crate::model::{
    layer110::token::{Token, TokenType},
    layer225::InlineTable,
};
use crate::parser::phase200::{
    layer210::PResult,
    layer220::usize_to_i128,
    layer225::{InlineTableP, KeyValueP},
};
use crate::util::random_name;
use casual_logger::Table as LogTable;

/// Inline table syntax machine state.  
/// インライン・テーブル構文状態遷移。  
///
/// Example: `{ key = value, key = value }`.  
#[derive(Debug)]
pub enum State {
    // First. After `{`.
    First,
    KeyValue,
    AfterKeyValue,
}

impl Default for InlineTableP {
    fn default() -> Self {
        InlineTableP {
            state: State::First,
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
            // After `{`.
            State::First => {
                match token.type_ {
                    TokenType::WhiteSpace => {} // Ignore it.
                    // `apple.banana`
                    TokenType::KeyWithoutDot => {
                        self.key_value_p = Some(Box::new(KeyValueP::new(&token)));
                        self.state = State::KeyValue;
                    }
                    TokenType::RightCurlyBracket => {
                        // Empty inline-table.
                        return PResult::End;
                    }
                    _ => return self.err_parse_first_otherwise(token),
                }
            }
            // `apple.banana`.
            State::KeyValue => {
                let p = self.key_value_p.as_mut().unwrap();
                match p.parse(token) {
                    PResult::End => {
                        if let Some(child_m) = p.flush() {
                            self.buffer.as_mut().unwrap().push_key_value(&child_m);
                            self.key_value_p = None;
                            self.state = State::AfterKeyValue;
                        } else {
                            return self.err_parse_key_value_empty_flush(token);
                        }
                    }
                    PResult::Err(mut table) => {
                        return self.err_parse_key_value_via(token, &mut table)
                    }
                    PResult::Ongoing => {}
                }
            }
            // After `banana = 3`.
            State::AfterKeyValue => match token.type_ {
                TokenType::WhiteSpace => {} // Ignore it.
                // `,`
                TokenType::Comma => {
                    self.state = State::First;
                }
                // `}`
                TokenType::RightCurlyBracket => {
                    return PResult::End;
                }
                _ => return self.err_parse_after_key_value_otherwise(token),
            },
        }
        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log_snapshot(&self) -> LogTable {
        let mut t = LogTable::default()
            .str("parser", "InlineTableP#parse")
            .str("state", &format!("{:?}", self.state))
            .clone();
        if let Some(key_value_p) = &self.key_value_p {
            t.sub_t("key_value", &key_value_p.log_snapshot());
        }
        t
    }

    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_first_otherwise(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "inline_table/err_parse_first_otherwise",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("state", &format!("{:?}", self.state))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }

    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_key_value_empty_flush(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "inline_table.rs/err_parse_key_value_empty_flush",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }

    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_key_value_via(&self, token: &Token, table: &mut LogTable) -> PResult {
        PResult::Err(
            table
                .sub_t(
                    &random_name(),
                    self.log_snapshot()
                        .str("via", "inline_table.rs/err_parse_key_value_via")
                        .int("column_number", usize_to_i128(token.column_number))
                        .str("token", &format!("{:?}", token)),
                )
                .clone(),
        )
    }

    /// Error message.  
    /// エラー・メッセージ。  
    fn err_parse_after_key_value_otherwise(&self, token: &Token) -> PResult {
        PResult::Err(
            self.log_snapshot()
                .str(
                    "place_of_occurrence",
                    "inline_table.rs/err_parse_after_key_value_otherwise",
                )
                .int("column_number", usize_to_i128(token.column_number))
                .str("token", &format!("{:?}", token))
                .clone(),
        )
    }
}
