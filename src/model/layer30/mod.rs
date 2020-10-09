pub mod array_of_table;
pub mod broad_line;
pub mod table;

use crate::model::{layer10::Comment, layer20::KeyValue};

/// WIP.  
#[derive(Clone)]
pub struct ArrayOfTable {
    pub value: String,
}

/// Either a Empty-line, Comment, Key Value, Table or a Array-of-table.  
/// 空行、コメント、キー値、テーブル、テーブルの配列のいずれかです。  
#[derive(Clone)]
pub enum BroadLine {
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
