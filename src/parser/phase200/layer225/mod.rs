pub mod inline_table;
pub mod key_value;

use crate::model::{
    layer110::token::Token,
    layer220::{InlineTable, KeyValue},
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, SingleQuotedStringP},
    layer220::ArrayP,
    layer225::{inline_table::State as InlineTableState, key_value::State as KeyValueState},
};

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
