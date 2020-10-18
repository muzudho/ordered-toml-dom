pub mod basic_string_p;
pub mod comment_p;
pub mod escape_sequence_p;
pub mod header_p_of_array_of_table;
pub mod header_p_of_table;
pub mod key_p;
pub mod literal_string_p;
pub mod literal_value_p;

use crate::model::{
    layer210::{BasicString, Comment, Key, LiteralString, LiteralValue},
    layer230::{HeaderOfArrayOfTable, HeaderOfTable},
};
use crate::parser::phase200::layer210::{
    basic_string_p::State as BasicStringState, escape_sequence_p::State as EscapeSequenceState,
    literal_string_p::State as LiteralStringState,
};
use crate::parser::phase200::Token;
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
pub struct BasicStringP {
    escape_sequence_p: Option<EscapeSequenceP>,
    buffer: Option<BasicString>,
    state: BasicStringState,
}

/// Escape sequence parser.  
/// エスケープ・シーケンス・パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct EscapeSequenceP {
    buffer: Vec<Token>,
    state: EscapeSequenceState,
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

/// Key parser.  
/// キー・パーサー。  
///
/// Example: `abc`.  
#[derive(Clone)]
pub struct KeyP {
    buffer: Option<Key>,
}

/// Literal string syntax parser.  
/// リテラル文字列構文パーサー。  
///
/// Example: `abc`.  
#[derive(Clone)]
pub struct LiteralValueP {
    buffer: Option<LiteralValue>,
}

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum PResult {
    /// End of syntax.
    End,
    // EndCarryOver(Token),
    Ongoing,
    /// Error.
    Err(LogTable),
}

/// Single quoted string syntax parser.  
/// 単一引用符文字列構文パーサー。  
///
/// Example: `'value'`.  
#[derive(Clone)]
pub struct LiteralStringP {
    buffer: Option<LiteralString>,
    state: LiteralStringState,
}
