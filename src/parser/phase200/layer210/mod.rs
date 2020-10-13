pub mod comment;
pub mod double_quoted_string;
pub mod header_of_array_of_table;
pub mod header_of_table;
pub mod literal_string;
pub mod single_quoted_string;

use crate::model::{
    layer210::{Comment, DoubleQuotedString, LiteralString, SingleQuotedString},
    layer230::{HeaderOfArrayOfTable, HeaderOfTable},
};
use crate::parser::phase200::layer210::double_quoted_string::State as DoubleQuotedStringState;
use casual_logger::Table as LogTable;

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
    state: DoubleQuotedStringState,
}

/// Header of array of table syntax parser.  
/// テーブル配列ヘッダー構文パーサー。  
///
/// Example: `[[value]]`.  
#[derive(Clone)]
pub struct HeaderPOfArrayOfTable {
    buffer: Option<HeaderOfArrayOfTable>,
}

/// Header of table syntax parser.  
/// テーブル・ヘッダー構文パーサー。  
///
/// Example: `[value]`.  
#[derive(Clone)]
pub struct HeaderPOfTable {
    buffer: Option<HeaderOfTable>,
}

/// Literal string syntax parser.  
/// リテラル文字列構文パーサー。  
///
/// Example: `abc`.  
#[derive(Clone)]
pub struct LiteralStringP {
    buffer: Option<LiteralString>,
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
