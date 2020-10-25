pub mod expression_p;

use crate::model::layer230::Expression;
use crate::parser::phase200::layer210::WSP;
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable},
    layer225::KeyvalP,
    layer230::expression_p::State as ExpressionState,
};

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct ExpressionP {
    buffer: Option<Expression>,
    comment_p: Option<CommentP>,
    header_p_of_array_of_table: Option<HeaderPOfArrayOfTable>,
    header_p_of_table: Option<HeaderPOfTable>,
    keyval_p: Option<KeyvalP>,
    state: ExpressionState,
    /// White space parser 1.
    ws_p_1: Option<WSP>,
    /// White space parser 2.
    ws_p_2: Option<WSP>,
}
