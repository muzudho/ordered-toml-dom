//! Syntax parser.  
//! 構文パーサー。  

pub mod document_element;
pub mod key_value;
pub mod layer10;
pub mod layer20;
pub mod machine_state;

use crate::model::{layer20::KeyValue, layer30::DocumentElement};
use crate::parser::syntax::{
    layer10::{ArrayOfTableP, CommentP, DoubleQuotedStringP, SingleQuotedStringP, TableP},
    layer20::{ArrayP, InlineTableP},
    machine_state::{BroadLineState, KeyValueState},
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
