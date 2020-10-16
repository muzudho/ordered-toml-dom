pub mod array;

use crate::model::layer220::Array;
use crate::parser::phase200::{
    layer210::{DoubleQuotedStringP, LiteralStringP},
    layer220::array::State as ArrayState,
};
use std::convert::TryInto;

pub fn usize_to_i128(num: usize) -> i128 {
    if let Ok(n) = num.try_into() {
        n
    } else {
        -1
    }
}

/// Array parser.  
/// 配列パーサー。  
///
/// Example: `[ 'a', 'b', 'c' ]`.  
#[derive(Clone)]
pub struct ArrayP {
    buffer: Option<Array>,
    /// Recursive.
    array_p: Option<Box<ArrayP>>,
    double_quoted_string_p: Option<Box<DoubleQuotedStringP>>,
    single_quoted_string_p: Option<Box<LiteralStringP>>,
    state: ArrayState,
}
