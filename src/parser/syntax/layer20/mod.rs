pub mod array_of_table;
pub mod comment;
pub mod double_quoted_string;
pub mod single_quoted_string;
pub mod table;

use crate::model::{
    layer20::{Comment, DoubleQuotedString, SingleQuotedString},
    layer40::{ArrayOfTable, Table as TableM},
};
use casual_logger::Table as LogTable;

/// Array of table syntax parser.  
/// テーブル配列構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct ArrayOfTableP {
    buffer: Option<ArrayOfTable>,
}

/// Comment parser.  
/// コメント・パーサー。  
///
/// Example: `# comment`.  
#[derive(Clone)]
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

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum PResult {
    /// End of syntax.
    End,
    Ongoing,
    /// Error.
    Err(LogTable),
}

/// Single quoted string syntax parser.  
/// 単一引用符文字列構文パーサー。  
///
/// Example: `'value'`.  
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedString>,
}

/// Table syntax parser.  
/// テーブル構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct TableP {
    buffer: Option<TableM>,
}
