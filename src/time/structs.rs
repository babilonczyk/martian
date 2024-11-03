use thiserror::Error;

/// Errors that can occur while performing time-related operations.
#[derive(Error, Debug)]
pub enum TimeError {
    /// Unable to retrieve the current UTC time.
    #[error("Unable to retrieve the current UTC time.")]
    UtcTimeUnavailable,

    /// Unable to retrieve the official leap seconds.
    #[error(
        "Failed to retrieve the official leap seconds, which are required for accurate MSD calculation."
    )]
    LeapSecondError,

    /// The calculated Mars Sol Date (MSD) is invalid or out of expected bounds.
    #[error("The resulting Mars Sol Date (MSD) is invalid or out of expected bounds.")]
    CalculationError,
}
