//! Syntax parser.  
//! 構文解析器。  

pub mod layer210;
pub mod layer220;
pub mod layer225;
pub mod layer230;
pub mod layer310;

use crate::model::layer110::Token;
use crate::parser::phase200::layer210::PResult;
use crate::util::random_name;
use casual_logger::Table as LogTable;
use look_ahead_items::LookAheadItems;

/// Error message.  
/// エラー・メッセージ。  
///
/// # Arguments
///
/// * `characters` - Tokens contains look ahead.  
///             先読みを含むトークン。  
fn error(
    table: &mut LogTable,
    characters: &LookAheadItems<char>,
    place_of_occurrence: &str,
) -> PResult {
    table.str("place_of_occurrence", place_of_occurrence);

    table.str("characters", &format!("{}", characters));

    PResult::Err(table.clone())
}

/// Error message.  
/// エラー・メッセージ。  
///
/// # Arguments
///
/// * `characters` - Tokens contains look ahead.  
///             先読みを含むトークン。  
fn error_via(
    escalated_table1: &mut LogTable,
    this_table: &mut LogTable,
    characters: &LookAheadItems<char>,
    place_of_occurrence: &str,
) -> PResult {
    this_table.str("via", place_of_occurrence);

    this_table.str("characters", &format!("{}", characters));

    PResult::Err(escalated_table1.sub_t(&random_name(), this_table).clone())
}
