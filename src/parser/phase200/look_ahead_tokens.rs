//! Look-ahead tokens.  
//! 先読みトークン。  

use crate::parser::phase200::LookAheadTokens;
use crate::parser::phase200::Token;

impl Default for LookAheadTokens {
    fn default() -> Self {
        LookAheadTokens {
            current: None,
            one_ahead: None,
            two_ahead: None,
        }
    }
}
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

    pub fn push(&mut self, token: &Option<Token>) {
        self.current = self.one_ahead.clone();
        self.one_ahead = self.two_ahead.clone();
        self.two_ahead = token.clone();
    }
}
