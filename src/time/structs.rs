use thiserror::Error;
use std::fmt;

// ------------------------------------------------------------------------------------------------
/// Errors that can occur while performing time-related operations.
#[derive(Error, Debug, PartialEq)]
pub enum TimeError {
    /// Unable to retrieve the current UTC time.
    #[error("Unable to retrieve the current UTC time.")]
    UtcTimeUnavailable,

    /// Unable to retrieve the official leap seconds.
    #[error(
        "Failed to retrieve the official leap seconds, which are required for accurate calculation."
    )]
    LeapSecondError,

    /// The calculated time is invalid or out of expected bounds.
    #[error("The result of time calulation is invalid or out of expected bounds.")]
    TimeCalculationError,

    /// The provided argument value is invalid.
    #[error("The provided argument value is invalid.")]
    InvalidArgumentError,

    /// Provided date does not match the ISO8601 format. Eg. 2021-08-06T05:17:57.000
    #[error("Provided date does not match the ISO8601 format. Eg. 2021-08-06T05:17:57.000")]
    ISO8601FormatError,

    /// Cannot provide a date below Sol 0 (1873-12-29T00:00:00.000 UTC).
    #[error("Cannot provide a date below Sol 0 (1873-12-29T00:00:00.000 UTC).")]
    DateBelowSolZeroError,
}

// ------------------------------------------------------------------------------------------------
/// Represents a time with hours, minutes, seconds, and milliseconds.
#[derive(Debug, PartialEq, Eq)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
}

impl Time {
    /// Constructs a new `Time` instance with the provided hours, minutes, seconds, and milliseconds.
    pub fn new(hours: u32, minutes: u32, seconds: u32, milliseconds: u32) -> Self {
        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}:{:03}",
            self.hours,
            self.minutes,
            self.seconds,
            self.milliseconds
        )
    }
}
