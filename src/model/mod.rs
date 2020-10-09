pub mod array;
pub mod array_of_table;
pub mod broad_line;
pub mod document;
pub mod inline_table;
pub mod item_value;
pub mod key_value;
pub mod layer1;
pub mod right_value;
pub mod table;

use crate::model::layer1::{Comment, DoubleQuotedString, LiteralString, SingleQuotedString};

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct Array {
    items: Vec<ItemValue>,
}

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

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct InlineTable {
    items: Vec<ItemValue>,
}

/// Array, inline table item.  
/// 配列、インライン・テーブルの項目です。  
#[derive(Clone)]
pub enum ItemValue {
    Array(Array),
    DoubleQuotedString(DoubleQuotedString),
    InlineTable(InlineTable),
    KeyValue(KeyValue),
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}

/// It has a key and a value.  
/// キーと値を持ちます。  
#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Box<RightValue>,
}

/// The right side of the key value model.  
/// キー値モデルの右辺です。  
#[derive(Clone)]
pub enum RightValue {
    Array(Array),
    DoubleQuotedString(DoubleQuotedString),
    InlineTable(InlineTable),
    // No KeyValue.
    LiteralString(LiteralString),
    SingleQuotedString(SingleQuotedString),
}

/// WIP.  
#[derive(Clone)]
pub struct Table {
    pub value: String,
}
