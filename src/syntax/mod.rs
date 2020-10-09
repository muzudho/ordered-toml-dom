//! Syntax parser.  
//! 構文パーサー。  

pub mod array;
pub mod array_of_table;
pub mod broad_line;
pub mod inline_table;
pub mod key_value;
pub mod layer10;
pub mod machine_state;
pub mod single_quoted_string;
pub mod table;

use crate::model::{
    layer10::SingleQuotedString,
    layer20::{Array, InlineTable, KeyValue},
    layer30::{ArrayOfTable, BroadLine, Table as TableM},
};
use crate::syntax::{
    layer10::{CommentP, DoubleQuotedStringP},
    machine_state::{ArrayState, BroadLineState, InlineTableState, KeyValueState},
};
use crate::token::Token;
use casual_logger::Table as LogTable;
use std::convert::TryInto;

fn usize_to_i128(num: usize) -> i128 {
    if let Ok(n) = num.try_into() {
        n
    } else {
        -1
    }
}

/// Result of syntax parser.  
/// 構文パーサーの結果。  
pub enum SyntaxParserResult {
    /// End of syntax.
    End,
    Ongoing,
    /// Error.
    Err(LogTable),
}

/// Array of table syntax parser.  
/// テーブル配列構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct ArrayOfTableP {
    buffer: Option<ArrayOfTable>,
}

/// Array parser.  
/// 配列パーサー。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone)]
pub struct ArrayP {
    buffer: Option<Array>,
    double_quoted_string_p: Option<Box<DoubleQuotedStringP>>,
    single_quoted_string_p: Option<Box<SingleQuotedStringP>>,
    state: ArrayState,
}

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct BroadLineP {
    array_of_table_p: Option<ArrayOfTableP>,
    buffer: Option<BroadLine>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
    state: BroadLineState,
    table_p: Option<TableP>,
}

/// Inline table syntax parser.  
/// インライン・テーブル構文パーサー。  
///
/// Example: `{ key = value, key = value }`.  
pub struct InlineTableP {
    state: InlineTableState,
    buffer: Option<InlineTable>,
    key_value_p: Option<Box<KeyValueP>>,
}

/// Key value syntax parser.  
/// キー値構文パーサー。  
///
/// `key = value`.  
pub struct KeyValueP {
    array_p: Option<ArrayP>,
    buffer: Option<KeyValue>,
    double_quoted_string_p: Option<DoubleQuotedStringP>,
    inline_table_p: Option<InlineTableP>,
    single_quoted_string_p: Option<SingleQuotedStringP>,
    state: KeyValueState,
    temp_key: Token,
}

/// Single quoted string syntax parser.  
/// 単一引用符文字列構文パーサー。  
///
/// Example: `'value'`.  
#[derive(Clone)]
pub struct SingleQuotedStringP {
    buffer: Option<SingleQuotedString>,
}

/// Table syntax parser.  
/// テーブル構文パーサー。  
///
/// Example: `"value"`.  
#[derive(Clone)]
pub struct TableP {
    buffer: Option<TableM>,
}
