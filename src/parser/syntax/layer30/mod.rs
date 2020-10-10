pub mod document_element;

use crate::model::layer30::DocumentElement;
use crate::parser::syntax::{
    layer10::{ArrayOfTableP, CommentP, TableP},
    layer20::KeyValueP,
    layer30::document_element::DocumentElementState,
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
