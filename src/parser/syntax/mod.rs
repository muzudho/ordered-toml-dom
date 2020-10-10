//! Syntax parser.  
//! 構文パーサー。  

pub mod document_element;
pub mod layer10;
pub mod layer20;
pub mod machine_state;

use crate::model::layer30::DocumentElement;
use crate::parser::syntax::{
    layer10::{ArrayOfTableP, CommentP, TableP},
    layer20::KeyValueP,
    machine_state::BroadLineState,
};
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
