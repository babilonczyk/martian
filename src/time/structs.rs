use std::fmt;

// ------------------------------------------------------------------------------------------------
/// Represents a time value with hours, minutes, seconds, and milliseconds.
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
