//! Syntax parser.  
//! 構文解析器。  

pub mod layer210;
pub mod layer220;
pub mod layer225;
pub mod layer230;
pub mod layer310;

use crate::model::layer110::token::Token;
use crate::parser::phase200::layer210::PResult;
use crate::parser::phase200::layer220::usize_to_i128;
use crate::util::random_name;
use casual_logger::Table as LogTable;

/// Error message.  
/// エラー・メッセージ。  
fn error(table: &mut LogTable, token: &Token, place_of_occurrence: &str) -> PResult {
    PResult::Err(
        table
            .str("place_of_occurrence", place_of_occurrence)
            .int("column_number", usize_to_i128(token.column_number))
            .str("token", &format!("{:?}", token))
            .clone(),
    )
}

/// Error message.  
/// エラー・メッセージ。  
fn error_via(
    escalated_table1: &mut LogTable,
    this_table: &mut LogTable,
    token: &Token,
    place_of_occurrence: &str,
) -> PResult {
    PResult::Err(
        escalated_table1
            .sub_t(
                &random_name(),
                this_table
                    .str("via", place_of_occurrence)
                    .int("column_number", usize_to_i128(token.column_number))
                    .str("token", &format!("{:?}", token)),
            )
            .clone(),
    )
}
