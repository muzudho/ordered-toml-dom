pub mod lexical_parser;
use crate::model::layer110::{CharacterLine, CharacterType};
use crate::parser::phase100::lexical_parser::State as LexicalParserState;

/// Lexical parser.  
/// 字句解析器。  
pub struct LexicalParser {
    state: LexicalParserState,
    product: CharacterLine,
    buffer_character_column_number: usize,
    buffer_character_type: Option<CharacterType>,
}
