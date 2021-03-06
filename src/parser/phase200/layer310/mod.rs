pub mod document_p;

use crate::parser::phase200::layer230::ExpressionP;

/// Document syntax parser.  
/// ドキュメント構文解析器。  
pub struct DocumentP {
    pub expression_p: Option<ExpressionP>,
}
