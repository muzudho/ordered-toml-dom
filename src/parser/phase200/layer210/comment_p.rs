//! Comment syntax parser.  
//! コメント構文パーサー。  

use crate::model::{
    layer110::{CharacterType, TokenType},
    layer210::Comment,
};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::{non_eol_p::Judge as NonEolPJudge, NonEolP};
use crate::parser::phase200::layer210::{CommentP, PResult};
use crate::parser::phase200::Character;
use crate::parser::phase200::LookAheadCharacters;
use crate::parser::phase200::Token;
use casual_logger::Table;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    First,
    NonEol,
}

pub enum Judge {
    CommentStartSymbol,
    CommentCharacter(Character),
}

impl CommentP {
    pub fn new() -> Self {
        CommentP {
            product: Comment::default(),
            state: State::First,
        }
    }
    pub fn get_product(&mut self) -> Comment {
        self.product.clone()
    }
    /// # Arguments
    ///
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(&self, character: &Character) -> Option<Judge> {
        match self.state {
            State::End => None,
            State::First => match character.type_ {
                CharacterType::CommentStartSymbol => Some(Judge::CommentStartSymbol),
                _ => None,
            },
            State::NonEol => {
                if let Some(judge) = NonEolP::judge(character) {
                    match judge {
                        NonEolPJudge::Ascii(ch)
                        | NonEolPJudge::HorizontalTab(ch)
                        | NonEolPJudge::NonAscii(ch) => Some(Judge::CommentCharacter(ch)),
                    }
                } else {
                    None
                }
            }
        }
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, characters: &LookAheadCharacters) -> PResult {
        match self.state {
            State::End => {
                return error(&mut self.log(), &characters, "comment_p.rs.61.");
            }
            State::First => {
                let character0 = characters.current.as_ref().unwrap();

                if let Some(judge) = self.judge(character0) {
                    self.product
                        .push_token(&Token::from_character(&character0, TokenType::Comment));
                    self.state = State::NonEol;
                } else {
                    return error(&mut self.log(), &characters, "comment_p.rs.61.");
                }
            }
            State::NonEol => {
                let character0 = characters.current.as_ref().unwrap();

                if let Some(_judge) = self.judge(character0) {
                    self.product
                        .push_token(&Token::from_character(&character0, TokenType::Comment));
                } else {
                    self.state = State::End;
                    return PResult::End;
                }
            }
        }

        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        t.str("product", &self.product.to_string());
        t
    }
}
