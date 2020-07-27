//! Machine state.  
//! 状態遷移。  

/// Array syntax machine state.  
/// 配列構文状態遷移。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone, Debug)]
pub enum ArrayState {
    /// [ か , の次。
    AfterDoubleQuotedString,
    AfterLeftSquareBracket,
    AfterSingleQuotedString,
    /// , か ] を待ちます。
    AfterItem,
    DoubleQuotedString,
    End,
    SingleQuotedString,
}

/// Line syntax machine state.  
/// 行構文状態遷移。  
#[derive(Debug)]
pub enum LineState {
    AfterComment,
    AfterKeyValue,
    /// `# comment`.
    CommentSyntax,
    Finished,
    First,
    /// `key = right_value`.
    KeyValueSyntax,
    Unimplemented,
}

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
