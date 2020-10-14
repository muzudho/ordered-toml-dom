pub mod document_line_scanner;

use crate::parser::phase200::layer230::DocumentElementP;

/// Document syntax parser.  
/// ドキュメント構文解析器。  
pub struct DocumentLineScanner {
    pub document_element_p: DocumentElementP,
}
