pub mod array_of_table;
pub mod document_element;
pub mod table;

use crate::model::{layer20::Comment, layer30::KeyValue};

/// WIP.  
#[derive(Clone)]
pub struct ArrayOfTable {
    pub value: String,
}

/// Either a Empty-line, Comment, Key Value, Table or a Array-of-table.  
/// 空行、コメント、キー値、テーブル、テーブルの配列のいずれかです。  
#[derive(Clone)]
pub enum DocumentElement {
    ArrayOfTable(ArrayOfTable),
    Comment(Comment),
    EmptyLine,
    KeyValue(KeyValue),
    Table(Table),
}

/// WIP.  
#[derive(Clone)]
pub struct Table {
    pub value: String,
}
