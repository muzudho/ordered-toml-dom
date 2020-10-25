pub mod inline_table;
pub mod keyval;
pub mod val;

use crate::model::{
    layer210::{BasicString, Key, LiteralString, LiteralValue},
    layer220::Array,
};

/// It has a key and a value.  
/// キーと値を持ちます。  
#[derive(Clone)]
pub struct Keyval {
    pub key: Box<Key>,
    /// Right value. Recursive.
    /// 右値。 再帰的。
    pub val: Box<Val>,
}

/// The right side of the key value model.  
/// キー値モデルの右辺です。  
#[derive(Clone)]
pub enum Val {
    /// Recursive.
    /// 再帰的。
    Array(Array),
    BasicString(BasicString),
    /// Recursive.
    /// 再帰的。
    InlineTable(InlineTable),
    // No Keyval.
    LiteralValue(LiteralValue),
    LiteralString(LiteralString),
}

/// It has multiple key-values.  
/// 複数の キー・バリュー を持ちます。  
#[derive(Clone)]
pub struct InlineTable {
    items: Vec<Keyval>,
}
