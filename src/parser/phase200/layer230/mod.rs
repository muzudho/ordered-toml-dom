pub mod expression_p;

use crate::model::layer230::Expression;
use crate::parser::phase200::{
    layer210::{CommentP, HeaderPOfArrayOfTable, HeaderPOfTable},
    layer225::KeyValueP,
    layer230::expression_p::State as ExpressionState,
};

/// Broad-line syntax parser.  
/// `縦幅のある行` パーサー。  
pub struct ExpressionP {
    header_p_of_array_of_table: Option<HeaderPOfArrayOfTable>,
    buffer: Option<Expression>,
    comment_p: Option<CommentP>,
    key_value_p: Option<KeyValueP>,
    state: ExpressionState,
    header_p_of_table: Option<HeaderPOfTable>,
}
