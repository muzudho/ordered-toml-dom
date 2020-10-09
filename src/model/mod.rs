pub mod array_of_table;
pub mod broad_line;
pub mod document;
pub mod key_value;
pub mod layer10;
pub mod layer20;
pub mod right_value;
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

/// It has multiple `broad_line`.  
/// 複数の `縦幅を持つ行` を持ちます。  
#[derive(Clone)]
pub struct Document {
    /// Line with height.
    /// 縦幅を持つ行。
    pub broad_lines: Vec<BroadLine>,
}

/// WIP.  
#[derive(Clone)]
pub struct Table {
    pub value: String,
}
