pub mod inline_table;
pub mod key_value;
pub mod right_value;

use crate::model::{
    layer210::{BasicString, Key, LiteralString, LiteralValue},
    layer220::Array,
};

/// It has a key and a value.  
/// キーと値を持ちます。  
#[derive(Clone)]
pub struct KeyValue {
    pub key: Box<Key>,
    /// Recursive.
    /// 再帰的。
    pub value: Box<RightValue>,
}

/// The right side of the key value model.  
/// キー値モデルの右辺です。  
#[derive(Clone)]
pub enum RightValue {
    /// Recursive.
    /// 再帰的。
    Array(Array),
    BasicString(BasicString),
    /// Recursive.
    /// 再帰的。
    InlineTable(InlineTable),
    // No KeyValue.
    LiteralValue(LiteralValue),
    LiteralString(LiteralString),
}

/// It has multiple key-values.  
/// 複数の キー・バリュー を持ちます。  
#[derive(Clone)]
pub struct InlineTable {
    items: Vec<KeyValue>,
}
