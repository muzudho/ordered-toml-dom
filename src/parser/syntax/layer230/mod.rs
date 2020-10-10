pub mod document_element;

use crate::model::layer230::DocumentElement;
use crate::parser::syntax::{
    layer210::{ArrayOfTableP, CommentP, TableP},
    layer220::KeyValueP,
    layer230::document_element::DocumentElementState,
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
