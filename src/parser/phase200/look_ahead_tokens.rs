//! Look-ahead tokens.  
//! 先読みトークン。  

use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::Token;

impl LookAheadTokens {
    pub fn from_tuple(tokens: (Option<&Token>, Option<&Token>, Option<&Token>)) -> Self {
        LookAheadTokens {
            current: if let Some(t) = tokens.0 {
                Some(t.clone())
            } else {
                None
            },
            one_ahead: if let Some(t) = tokens.1 {
                Some(t.clone())
            } else {
                None
            },
            two_ahead: if let Some(t) = tokens.2 {
                Some(t.clone())
            } else {
                None
            },
        }
    }
}
