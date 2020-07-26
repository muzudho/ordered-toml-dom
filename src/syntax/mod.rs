//! Syntax parser.  
//! 構文パーサー。  

pub mod array;
pub mod comment;
pub mod inline_table;
pub mod key_value;
pub mod line;
pub mod machine_state;
pub mod single_quoted_string;

use crate::model::{ArrayM, CommentM, ElementM, InlineTableM, KeyValueM, SingleQuotedStringM};
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
    state: ArrayState,
    buffer: Option<ArrayM>,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
}

/// `# comment`.
pub struct CommentP {
    buffer: Option<CommentM>,
}

/// `{ key = value, key = value }`.
pub struct InlineTableP {
    state: InlineTableState,
    buffer: Option<InlineTableM>,
    key_value_p: Option<Box<KeyValueP>>,
}

/// `key = value`.
pub struct KeyValueP {
    state: KeyValueState,
    temp_key: Token,
    buffer: Option<KeyValueM>,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    array_p: Option<ArrayP>,
}

pub struct LineP {
    state: LineState,
    buffer: Option<ElementM>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
}

/// `'value'`.
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedStringM>,
}
