pub mod inline_table_p;
pub mod key_value_p;
pub mod right_value_p;

use crate::model::{
    layer110::Token,
    layer225::{InlineTable, KeyValue, RightValue},
};
use crate::parser::phase200::{
    layer210::{BasicStringP, LiteralStringP, LiteralValueP},
    layer220::ArrayP,
    layer225::{
        inline_table_p::State as InlineTableState, key_value_p::State as KeyValueState,
        right_value_p::State as RightValueState,
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
    key: Token,
    right_value_p: Option<RightValueP>,
    state: KeyValueState,
}

/// Right value syntax parser.  
/// 右値構文パーサー。  
///
/// `key = this`.  
pub struct RightValueP {
    array_p: Option<ArrayP>,
    buffer: Option<RightValue>,
    basic_string_p: Option<BasicStringP>,
    inline_table_p: Option<InlineTableP>,
    literal_value_p: Option<LiteralValueP>,
    literal_string_p: Option<LiteralStringP>,
    state: RightValueState,
}
