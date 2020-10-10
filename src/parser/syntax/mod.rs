//! Syntax parser.  
//! 構文パーサー。  

pub mod array;
pub mod document_element;
pub mod inline_table;
pub mod key_value;
pub mod layer10;
pub mod machine_state;

use crate::model::{
    layer20::{Array, InlineTable, KeyValue},
    layer30::DocumentElement,
};
use crate::parser::syntax::{
    layer10::{ArrayOfTableP, CommentP, DoubleQuotedStringP, SingleQuotedStringP, TableP},
    machine_state::{ArrayState, BroadLineState, InlineTableState, KeyValueState},
};
use crate::token::Token;
use std::convert::TryInto;

fn usize_to_i128(num: usize) -> i128 {
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

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct DocumentElementP {
    array_of_table_p: Option<ArrayOfTableP>,
    buffer: Option<DocumentElement>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
    state: BroadLineState,
    table_p: Option<TableP>,
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
