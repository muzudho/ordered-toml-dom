//! Machine state.  
//! 状態遷移。  

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

/// Key value syntax machine state.  
/// キー値構文状態遷移。  
///
/// Example: `key = right_value`.  
#[derive(Debug)]
pub enum KeyValueState {
    AfterKey,
    AfterEquals,
    AfterLeftCurlyBracket,
    AfterLeftSquareBracket,
    DoubleQuotedString,
    SingleQuotedString,
    End,
}
