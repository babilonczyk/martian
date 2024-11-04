use thiserror::Error;
use std::fmt;

// ------------------------------------------------------------------------------------------------
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
