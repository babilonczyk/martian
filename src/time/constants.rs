/// Length of a Martian sol in Earth days.
pub const SOL_IN_EARTH_DAYS: f64 = 1.0274912517;

/// Julian Date where Mars Sol Date (MSD) is zero.
pub const JD_ON_SOL_ZERO: f64 = 2405522.0028779;

/// Regex pattern for ISO8601 date format used in the library (YYYY-MM-DDTHH:MM:SS.sss).
pub const ISO8601_REGEX: &str =
    r"^(\d{4,})-(\d{1,2})-(\d{1,2})T(\d{1,2}):(\d{1,2}):(\d{1,2})(?:\.(\d{1,}))?.*?$";
