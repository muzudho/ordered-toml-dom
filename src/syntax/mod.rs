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

/// `[ 'a', 'b', 'c' ]`.
#[derive(Clone)]
pub struct ArrayP {
    buffer: Option<Array>,
    double_quoted_string_p: Option<Box<DoubleQuotedStringP>>,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
    state: ArrayState,
}

/// `# comment`.
pub struct CommentP {
    buffer: Option<Comment>,
}

/// `"value"`.
#[derive(Clone)]
pub struct DoubleQuotedStringP {
    buffer: Option<DoubleQuotedString>,
}

/// `{ key = value, key = value }`.
pub struct InlineTableP {
    state: InlineTableState,
    buffer: Option<InlineTable>,
    key_value_p: Option<Box<KeyValueP>>,
}

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

pub struct LineP {
    state: LineState,
    buffer: Option<Element>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
}

/// `'value'`.
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedString>,
}
