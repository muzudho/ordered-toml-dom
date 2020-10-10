//! Syntax parser.  
//! 構文パーサー。  

pub mod document_element;
pub mod layer10;
pub mod layer20;

use crate::model::layer30::DocumentElement;
use crate::parser::syntax::{
    document_element::DocumentElementState,
    layer10::{ArrayOfTableP, CommentP, TableP},
    layer20::KeyValueP,
};

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct DocumentElementP {
    array_of_table_p: Option<ArrayOfTableP>,
    buffer: Option<DocumentElement>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
    state: DocumentElementState,
    table_p: Option<TableP>,
}
