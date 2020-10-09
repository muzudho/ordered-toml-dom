pub mod array;
pub mod inline_table;
pub mod item_value;
pub mod key_value;
pub mod right_value;

use crate::model::layer10::{DoubleQuotedString, LiteralString, SingleQuotedString};

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct Array {
    items: Vec<ItemValue>,
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
