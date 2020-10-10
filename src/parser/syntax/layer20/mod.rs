pub mod array;

use crate::model::layer20::Array;
use crate::parser::syntax::{
    layer10::{DoubleQuotedStringP, SingleQuotedStringP},
    machine_state::ArrayState,
};

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
