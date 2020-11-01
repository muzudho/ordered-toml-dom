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
    CommentText,
}

impl CommentP {
    pub fn new() -> Self {
        CommentP {
            buffer: None,
            state: State::First,
            non_eol_p: None,
        }
    }
    pub fn flush(&mut self) -> Option<Comment> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    /// # Arguments
    ///
    /// * `token` - Token.  
    ///             トークン。  
    /// # Returns
    ///
    /// * `bool` - このパーサーの対象とするトークンになる.  
    ///                             結果。
    pub fn judge(character: &Character) -> Option<Judge> {
        match character.type_ {
            CharacterType::CommentStartSymbol => Some(Judge::CommentStartSymbol),
            _ => {
                if let Some(judge) = NonEolP::judge(character) {
                    match judge {
                        NonEolPJudge::HorizontalTabAndAscii | NonEolPJudge::NonAscii => {
                            Some(Judge::CommentText)
                        }
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
            State::End => {}
            State::First => {
                let character0 = characters.current.as_ref().unwrap();

                if let Some(_judge) = Self::judge(character0) {
                    if let None = self.buffer {
                        self.buffer = Some(Comment::default());
                    }
                    let m = self.buffer.as_mut().unwrap();
                    m.push_token(&Token::from_character(&character0, TokenType::Comment));
                    self.non_eol_p = Some(NonEolP::default());
                    self.state = State::NonEol;
                } else {
                    return error(&mut self.log(), &characters, "comment_p.rs.61.");
                }
            }
            State::NonEol => {
                let p = self.non_eol_p.as_mut().unwrap();
                match p.parse(characters) {
                    PResult::End => {
                        let m = self.buffer.as_mut().unwrap();
                        m.extend_tokens(&p.flush().unwrap().tokens);

                        self.non_eol_p = None;
                        self.state = State::End;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &characters,
                            "comment_p.rs.87.",
                        );
                    }
                    PResult::Ongoing => {
                        return PResult::Ongoing;
                    }
                }
            }
        }

        PResult::Ongoing
    }
    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
