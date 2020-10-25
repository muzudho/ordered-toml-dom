pub mod inline_table_p;
pub mod keyval_p;
pub mod val_p;

use crate::model::{
    layer210::Key,
    layer225::{InlineTable, Val},
};
use crate::parser::phase200::{
    layer210::{BasicStringP, KeyP, LiteralStringP, LiteralValueP},
    layer220::ArrayP,
    layer225::{
        inline_table_p::State as InlineTableState, keyval_p::State as KeyvalState,
        val_p::State as ValState,
    },
};

/// Inline table syntax parser.  
/// インライン・テーブル構文パーサー。  
///
/// Example: `{ key = value, key = value }`.  
pub struct InlineTableP {
    state: InlineTableState,
    buffer: Option<InlineTable>,
    keyval_p: Option<Box<KeyvalP>>,
}

/// Key value syntax parser.  
/// キー値構文パーサー。  
///
/// `key = value`.  
pub struct KeyvalP {
    key_buffer: Option<Key>,
    val_buffer: Option<Val>,
    key_p: Option<KeyP>,
    val_p: Option<ValP>,
    state: KeyvalState,
}

/// Right value syntax parser.  
/// 右値構文パーサー。  
///
/// `key = this`.  
pub struct ValP {
    array_p: Option<ArrayP>,
    buffer: Option<Val>,
    basic_string_p: Option<BasicStringP>,
    inline_table_p: Option<InlineTableP>,
    literal_value_p: Option<LiteralValueP>,
    literal_string_p: Option<LiteralStringP>,
    state: ValState,
}
