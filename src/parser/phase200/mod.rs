//! Syntax parser.  
//! 構文解析器。  

pub mod layer210;
pub mod layer220;
pub mod layer225;
pub mod layer230;
pub mod layer310;
pub mod look_ahead_tokens;

use crate::model::layer110::Token;
use crate::parser::phase200::layer210::PResult;
use crate::parser::phase200::layer220::usize_to_i128;
use crate::util::random_name;
use casual_logger::Table as LogTable;

pub struct LookAheadTokens {
    pub current: Option<Token>,
    pub one_ahead: Option<Token>,
    pub two_ahead: Option<Token>,
    pub three_ahead: Option<Token>,
    /// `2020-10-21` を `2020-` のハイフンまで先読みするのに使います。
    pub four_ahead: Option<Token>,
}

/// Error message.  
/// エラー・メッセージ。  
///
/// # Arguments
///
/// * `tokens` - Tokens contains look ahead.  
///             先読みを含むトークン。  
fn error(table: &mut LogTable, tokens: &LookAheadTokens, place_of_occurrence: &str) -> PResult {
    table.str("place_of_occurrence", place_of_occurrence);

    if let Some(token) = &tokens.current {
        table
            .int("token0_column_number", usize_to_i128(token.column_number))
            .str("token0", &format!("{}", token));
    }

    if let Some(token) = &tokens.one_ahead {
        table
            .int("token1_column_number", usize_to_i128(token.column_number))
            .str("token1", &format!("{}", token));
    }

    if let Some(token) = &tokens.two_ahead {
        table
            .int("token2_column_number", usize_to_i128(token.column_number))
            .str("token2", &format!("{}", token));
    }

    PResult::Err(table.clone())
}

/// Error message.  
/// エラー・メッセージ。  
///
/// # Arguments
///
/// * `tokens` - Tokens contains look ahead.  
///             先読みを含むトークン。  
fn error_via(
    escalated_table1: &mut LogTable,
    this_table: &mut LogTable,
    tokens: &LookAheadTokens,
    place_of_occurrence: &str,
) -> PResult {
    this_table.str("via", place_of_occurrence);

    if let Some(token) = &tokens.current {
        this_table
            .int("token0_column_number", usize_to_i128(token.column_number))
            .str("token0", &format!("{}", token));
    }

    if let Some(token) = &tokens.one_ahead {
        this_table
            .int("token1_column_number", usize_to_i128(token.column_number))
            .str("token1", &format!("{}", token));
    }

    if let Some(token) = &tokens.two_ahead {
        this_table
            .int("token2_column_number", usize_to_i128(token.column_number))
            .str("token2", &format!("{}", token));
    }

    PResult::Err(escalated_table1.sub_t(&random_name(), this_table).clone())
}
