//! Syntax parser.  
//! 構文パーサー。  

pub mod comment;
pub mod inline_table;
pub mod key_value;
pub mod line;
pub mod single_quoted_string;
use casual_logger::Table;

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum SyntaxParserResult {
    /// * `bool` - End of syntax.
    Ok(bool),
    /// Error.
    Err(Table),
}
