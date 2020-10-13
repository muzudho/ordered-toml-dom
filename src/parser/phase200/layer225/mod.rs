pub mod inline_table;
pub mod key_value;
pub mod right_value;

use crate::model::{
    layer110::token::Token,
    layer225::{InlineTable, KeyValue, RightValue},
};
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, SingleQuotedStringP},
    layer220::ArrayP,
    layer225::{
        inline_table::State as InlineTableState, key_value::State as KeyValueState,
        right_value::State as RightValueState,
    },
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
    buffer: Option<KeyValue>,
    right_value_p: Option<RightValueP>,
    state: KeyValueState,
    temp_key: Token,
}

/// Right value syntax parser.  
/// 右値構文パーサー。  
///
/// `key = this`.  
pub struct RightValueP {
    array_p: Option<ArrayP>,
    buffer: Option<RightValue>,
    double_quoted_string_p: Option<DoubleQuotedStringP>,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    state: RightValueState,
}
