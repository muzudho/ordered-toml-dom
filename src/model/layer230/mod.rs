pub mod document_element;
pub mod header_of_array_of_table;
pub mod header_of_table;

use crate::model::{layer210::Comment, layer220::KeyValue};

/// WIP.  
#[derive(Clone)]
pub struct HeaderOfArrayOfTable {
    pub value: String,
}

/// Either a Empty-line, Comment, Key Value, Table or a Array-of-table.  
/// 空行、コメント、キー値、テーブル、テーブルの配列のいずれかです。  
#[derive(Clone)]
pub enum DocumentElement {
    HeaderOfArrayOfTable(HeaderOfArrayOfTable),
    Comment(Comment),
    EmptyLine,
    KeyValue(KeyValue),
    HeaderOfTable(HeaderOfTable),
}

/// WIP.  
#[derive(Clone)]
pub struct HeaderOfTable {
    pub value: String,
}
