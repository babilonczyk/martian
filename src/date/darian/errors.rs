use thiserror::Error;
use crate::time::{ TimeError };

// ------------------------------------------------------------------------------------------------
/// Errors that can occur while performing date-related operations.
#[derive(Error, Debug, PartialEq)]
pub enum DateError {
    /// Propagated time error from the time module.
    #[error("Time error occurred: {0}")]
    TimeError(#[from] TimeError),

    /// Unable to provide month value below 1 or above 24.
    #[error("Unable to provide month value below or above 24")]
    MonthValueOutOfRange,

    /// Provided sol value is out of range.
    #[error("Provided sol value is out of range")]
    SolValueOutOfRange,

    /// Unable to convert to Utc date time.
    #[error("Unable to convert to Utc date time")]
    UtcConversionError,
}
