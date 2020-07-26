//! Syntax parser.  
//! 構文パーサー。  

pub mod array;
pub mod comment;
pub mod double_quoted_string;
pub mod inline_table;
pub mod key_value;
pub mod line;
pub mod machine_state;
pub mod single_quoted_string;

use crate::model::{
    Array, Comment, DoubleQuotedString, Element, InlineTable, KeyValue, SingleQuotedString,
};
use crate::syntax::machine_state::{ArrayState, InlineTableState, KeyValueState, LineState};
use crate::token::Token;
use casual_logger::Table;

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum SyntaxParserResult {
    /// End of syntax.
    End,
    Ongoing,
    /// Error.
    Err(Table),
}

/// Array parser.  
/// 配列パーサー。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone)]
pub struct ArrayP {
    buffer: Option<Array>,
    double_quoted_string_p: Option<Box<DoubleQuotedStringP>>,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
    state: ArrayState,
}

/// Comment parser.  
/// コメント・パーサー。  
///
/// Example: `# comment`.  
pub struct CommentP {
    buffer: Option<Comment>,
}

/// Double quoted string syntax parser.  
/// 二重引用符文字列構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct DoubleQuotedStringP {
    buffer: Option<DoubleQuotedString>,
}

/// Inline table syntax parser.  
/// インライン・テーブル構文パーサー。  
///
/// Example: `{ key = value, key = value }`.  
pub struct InlineTableP {
    state: InlineTableState,
    buffer: Option<InlineTable>,
    key_value_p: Option<Box<KeyValueP>>,
}

/// Key value syntax parser.  
/// キー値構文パーサー。  
///
/// `key = value`.  
pub struct KeyValueP {
    array_p: Option<ArrayP>,
    buffer: Option<KeyValue>,
    double_quoted_string_p: Option<DoubleQuotedStringP>,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    state: KeyValueState,
    temp_key: Token,
}

/// Line syntax parser.  
/// 行構文パーサー。  
pub struct LineP {
    state: LineState,
    buffer: Option<Element>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
}

/// Single quoted string syntax parser.  
/// 単一引用符文字列構文パーサー。  
///
/// Example: `'value'`.  
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedString>,
}
