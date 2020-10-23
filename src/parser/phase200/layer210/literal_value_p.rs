//! Litera value parser.  
//! リテラル値パーサー。  

use crate::model::layer110::token::tokens_stringify;
use crate::model::{
    layer110::{Token, TokenType},
    layer210::LiteralValue,
};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::DateTimeP;
use crate::parser::phase200::layer210::PositionalNumeralStringP;
use crate::parser::phase200::layer210::{
    date_time_p::State as DateTimeState, LiteralValueP, PResult,
};
use crate::parser::phase200::LookAheadTokens;
use casual_logger::Table as LogTable;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    DateTime,
    End,
    First,
    Second,
    /// 0x
    ZeroXPrefix1st,
    ZeroXString,
}

impl Default for LiteralValueP {
    fn default() -> Self {
        LiteralValueP {
            date_time_p: None,
            positional_numeral_string_p: None,
            buffer: Some(LiteralValue::default()),
            state: State::First,
        }
    }
}
impl LiteralValueP {
    pub fn flush(&mut self) -> Option<LiteralValue> {
        if let Some(literal_value) = &self.buffer {
            let m = Some(literal_value.clone()); // TODO トリム要らないのでは。
            self.buffer = None;
            return m;
        }
        None
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(&mut self, tokens: &LookAheadTokens) -> PResult {
        let token0 = tokens.current.as_ref().unwrap();
        match self.state {
            State::DateTime => {
                let p = self.date_time_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        let string_buffer = tokens_stringify(&p.flush());
                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::new(
                            token0.column_number,
                            &string_buffer,
                            TokenType::SPDateTimeString,
                        ));
                        self.date_time_p = None;
                        self.state = State::End;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "literal_value_p.rs.90.",
                        );
                    }
                    PResult::Ongoing => PResult::Ongoing,
                }
            }
            State::End => {
                return error(&mut self.log(), &tokens, "literal_value.rs.57.");
            }
            State::First => {
                // println!("[trace61 token0.type_={:?}]", &token0.type_);

                // TODO まず日付型かどうか調べると楽そう。
                // ２文字先を最初に調べるのがコツ。
                let mut is_date = true;
                let mut is_time = true;
                // `??n??`.
                if let Some(token2) = tokens.two_ahead.as_ref() {
                    if let Some(ch2) = token2.to_string().chars().nth(0) {
                        match ch2 {
                            '0'..='9' => {
                                is_time = false;
                            }
                            ':' => {
                                is_date = false;
                            }
                            _ => {
                                is_date = false;
                                is_time = false;
                            }
                        }
                    }
                } else {
                    is_date = false;
                    is_time = false;
                }
                if is_date {
                    // `??n?-`.
                    if let Some(token4) = tokens.four_ahead.as_ref() {
                        if let Some(ch4) = token4.to_string().chars().nth(0) {
                            if ch4 != '-' {
                                is_date = false;
                            }
                        }
                    }
                    // `??nn-`.
                    if is_date {
                        if let Some(token3) = tokens.three_ahead.as_ref() {
                            if let Some(ch3) = token3.to_string().chars().nth(0) {
                                match ch3 {
                                    '0'..='9' => {}
                                    _ => {
                                        is_date = false;
                                    }
                                }
                            }
                        }
                    }
                    // `?nnn-`.
                    if is_date {
                        if let Some(token1) = tokens.one_ahead.as_ref() {
                            if let Some(ch1) = token1.to_string().chars().nth(0) {
                                match ch1 {
                                    '0'..='9' => {}
                                    _ => {
                                        is_date = false;
                                    }
                                }
                            }
                        }
                    }
                    // `nnnn-`.
                    if is_date {
                        if let Some(ch0) = token0.to_string().chars().nth(0) {
                            match ch0 {
                                '0'..='9' => {
                                    // 日付型なのは確定。
                                    println!("trace126.日付型確定。");
                                    self.state = State::DateTime;
                                    self.date_time_p =
                                        Some(DateTimeP::new(DateTimeState::FirstOfDate));

                                    let p = self.date_time_p.as_mut().unwrap();
                                    match p.parse(&tokens) {
                                        PResult::End => {
                                            return error(
                                                &mut self.log(),
                                                &tokens,
                                                "literal_value_p.rs.170.",
                                            );
                                        }
                                        PResult::Err(mut table) => {
                                            return error_via(
                                                &mut table,
                                                &mut self.log(),
                                                &tokens,
                                                "literal_value_p.rs.178.",
                                            );
                                        }
                                        PResult::Ongoing => {}
                                    }
                                }
                                _ => {
                                    is_date = false;
                                }
                            }
                        }
                    }
                }
                if is_time {
                    // `?n:`.
                    if is_date {
                        if let Some(token1) = tokens.one_ahead.as_ref() {
                            if let Some(ch1) = token1.to_string().chars().nth(0) {
                                match ch1 {
                                    '0'..='9' => {}
                                    _ => {
                                        is_date = false;
                                    }
                                }
                            }
                        }
                    }
                    // `nn:`.
                    if is_date {
                        if let Some(ch0) = token0.to_string().chars().nth(0) {
                            match ch0 {
                                '0'..='9' => {
                                    // 時刻型なのは確定。
                                    println!("trace154.時刻型確定。");
                                    self.state = State::DateTime;
                                    self.date_time_p =
                                        Some(DateTimeP::new(DateTimeState::FirstOfTime));

                                    let p = self.date_time_p.as_mut().unwrap();
                                    match p.parse(&tokens) {
                                        PResult::End => {
                                            return error(
                                                &mut self.log(),
                                                &tokens,
                                                "literal_value_p.rs.222.",
                                            );
                                        }
                                        PResult::Err(mut table) => {
                                            return error_via(
                                                &mut table,
                                                &mut self.log(),
                                                &tokens,
                                                "literal_value_p.rs.230.",
                                            );
                                        }
                                        PResult::Ongoing => {}
                                    }
                                }
                                _ => {
                                    is_date = false;
                                }
                            }
                        }
                    }
                }

                if is_date || is_time {
                    PResult::Ongoing
                } else {
                    let base_number = match token0.type_ {
                        TokenType::AbChar
                        | TokenType::Colon
                        | TokenType::Dot
                        | TokenType::Hyphen
                        | TokenType::Plus
                        | TokenType::Underscore => 10,
                        TokenType::NumChar => {
                            if let Some(ch0) = token0.to_string().chars().nth(0) {
                                // println!("[trace82 ch0={}]", ch0);
                                if ch0 == '0' {
                                    // 0x ?
                                    // Look-ahead.
                                    // 先読み。
                                    if let Some(token1) = tokens.one_ahead.as_ref() {
                                        match token1.type_ {
                                            TokenType::AbChar => {
                                                match token1.to_string().as_str() {
                                                    "b" => 2,
                                                    "o" => 8,
                                                    "x" => 16,
                                                    _ => 10,
                                                }
                                            }
                                            _ => 10,
                                        }
                                    } else {
                                        10
                                    }
                                } else {
                                    10
                                }
                            } else {
                                10
                            }
                        }
                        _ => return error(&mut self.log(), &tokens, "literal_value_p.rs.38."),
                    };

                    match base_number {
                        2 => {
                            self.positional_numeral_string_p =
                                Some(PositionalNumeralStringP::new("0b").clone());
                            self.state = State::ZeroXPrefix1st;
                            PResult::Ongoing
                        }
                        8 => {
                            self.positional_numeral_string_p =
                                Some(PositionalNumeralStringP::new("0o").clone());
                            self.state = State::ZeroXPrefix1st;
                            PResult::Ongoing
                        }
                        16 => {
                            // `0x` は無視します。
                            // println!("[trace129={}]", token0);
                            self.positional_numeral_string_p =
                                Some(PositionalNumeralStringP::new("0x").clone());
                            self.state = State::ZeroXPrefix1st;
                            PResult::Ongoing
                        }
                        10 => {
                            let m = self.buffer.as_mut().unwrap();
                            m.push_token(&token0);
                            // Look-ahead.
                            // 先読み。
                            if let Some(token1) = &tokens.one_ahead {
                                match token1.type_ {
                                    TokenType::AbChar
                                    | TokenType::Colon
                                    | TokenType::Dot
                                    | TokenType::Hyphen
                                    | TokenType::NumChar
                                    | TokenType::Plus
                                    | TokenType::Underscore => {
                                        self.state = State::Second;
                                        PResult::Ongoing
                                    }
                                    _ => {
                                        self.state = State::End;
                                        PResult::End
                                    }
                                }
                            } else {
                                self.state = State::End;
                                PResult::End
                            }
                        }
                        _ => panic!("Err.170.Unimplemented."),
                    }
                }
            }
            State::Second => {
                // 10進数のみです。
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
                // Look-ahead.
                // 先読み。
                if let Some(token1) = &tokens.one_ahead {
                    match token1.type_ {
                        TokenType::AbChar
                        | TokenType::Colon
                        | TokenType::Dot
                        | TokenType::Hyphen
                        | TokenType::NumChar
                        | TokenType::Plus
                        | TokenType::Underscore => PResult::Ongoing,
                        _ => {
                            self.state = State::End;
                            PResult::End
                        }
                    }
                } else {
                    self.state = State::End;
                    PResult::End
                }
            }
            State::ZeroXPrefix1st => {
                // ここで トークンを文字列でまとめていたとき、
                // 例えば `0xDEADBEEF` の場合、2文字目の `x` を取ろうとすると
                // `xDEADBEEF` と、まとまりで取ってしまい、溢れる分の後処理が手間取りました。
                // そこで、アルファベットは１トークンずつ取ることにしました。
                // println!("[trace160={}]", token0);
                self.state = State::ZeroXString;
                PResult::Ongoing
            }
            State::ZeroXString => {
                // println!("[trace164={}]", token0);
                let p = self.positional_numeral_string_p.as_mut().unwrap();
                match p.parse(&tokens) {
                    PResult::End => {
                        // Filled.
                        // 満ちたなら。

                        // 数値変換はせず、頭に `0x` などを付けます。
                        // borrow の制約から、まず flush してから prefix にアクセスします。
                        let n_string = tokens_stringify(&p.flush());
                        let numeral_string = &format!("{}{}", &p.prefix, n_string);

                        let m = self.buffer.as_mut().unwrap();
                        m.push_token(&Token::new(
                            token0.column_number,
                            &numeral_string,
                            TokenType::SPPositionalNumeralString,
                        ));

                        // println!("[trace187={}]", &m.to_string());
                        // println!("[trace188={:?}]", &m.to_string());

                        self.positional_numeral_string_p = None;
                        self.state = State::End;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &tokens,
                            "literal_value_p.rs.173.",
                        );
                    }
                    PResult::Ongoing => PResult::Ongoing,
                }
            }
        }
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> LogTable {
        let mut t = LogTable::default().clone();
        if let Some(m) = &self.buffer {
            t.str("buffer", &m.to_string());
        }
        t
    }
}
