pub mod array;
pub mod item_value;

use crate::model::{
    layer210::{BasicString, LiteralString, LiteralValue},
    layer225::{InlineTable, Keyval},
};

/// It has multiple item values.  
/// 複数の項目値を持ちます。  
#[derive(Clone)]
pub struct Array {
    items: Vec<ItemValue>,
}

/// Array, inline table item.  
/// 配列、インライン・テーブルの項目です。  
#[derive(Clone)]
pub enum ItemValue {
    /// Recursive.
    /// 再帰的。
    Array(Array),
    BasicString(BasicString),
    /// Recursive.
    /// 再帰的。
    InlineTable(InlineTable),
    /// Recursive.
    /// 再帰的。
    Keyval(Keyval),
    LiteralValue(LiteralValue),
    LiteralString(LiteralString),
}
