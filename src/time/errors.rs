use thiserror::Error;

// ------------------------------------------------------------------------------------------------
/// Errors that can occur while performing time-related operations.
#[derive(Error, Debug, PartialEq)]
pub enum TimeError {
    /// Unable to retrieve the current UTC time.
    #[error("Unable to retrieve the current UTC time.")]
    UtcTimeUnavailable,

    /// The calculated time is invalid or out of expected bounds.
    #[error("The result of time calulation is invalid or out of expected bounds.")]
    TimeCalculationError,

    /// The provided argument value is invalid.
    #[error("The provided argument value is invalid.")]
    InvalidArgumentError,

    /// Provided date does not match the ISO8601 format. Eg. 2021-08-06T05:17:57.000
    #[error("Provided date does not match the ISO8601 format. Eg. 2021-08-06T05:17:57.000")]
    ISO8601FormatError,
}
