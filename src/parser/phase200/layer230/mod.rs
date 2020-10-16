pub mod document_element_p;

use crate::model::layer230::DocumentElement;
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable},
    layer225::KeyValueP,
    layer230::document_element_p::State as DocumentElementState,
};

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct DocumentElementP {
    header_p_of_array_of_table: Option<HeaderPOfArrayOfTable>,
    buffer: Option<DocumentElement>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
    state: DocumentElementState,
    header_p_of_table: Option<HeaderPOfTable>,
}
