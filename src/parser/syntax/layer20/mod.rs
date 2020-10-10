pub mod array;
pub mod inline_table;
pub mod key_value;

use crate::model::{
    layer20::{Array, InlineTable, KeyValue},
    layer5::token::Token,
};
use crate::parser::syntax::{
    layer10::{DoubleQuotedStringP, SingleQuotedStringP},
    layer20::{array::ArrayState, inline_table::InlineTableState, key_value::KeyValueState},
};
use std::convert::TryInto;

pub fn usize_to_i128(num: usize) -> i128 {
    if let Ok(n) = num.try_into() {
        n
    } else {
        -1
    }
}

/// Array parser.  
/// 配列パーサー。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone)]
pub struct ArrayP {
    buffer: Option<Array>,
    double_quoted_string_p: Option<Box<DoubleQuotedStringP>>,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
    state: ArrayState,
}

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
