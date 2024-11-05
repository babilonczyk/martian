/// Length of a Martian sol in Earth days.
pub const SOL_IN_EARTH_DAYS: f64 = 1.0274912517;

/// Julian Date where Mars Sol Date (MSD) is zero.
pub const JD_ON_SOL_ZERO: f64 = 2405522.0028779;

/// Difference in seconds between TAI and TT.
pub const TAI_TO_TT_OFFSET: f64 = 32.184;

/// Regex pattern for ISO8601 date format used in the library (YYYY-MM-DDTHH:MM:SS.sss).
pub const ISO8601_REGEX: &str =
    r"^(\d{4,})-(\d{1,2})-(\d{1,2})T(\d{1,2}):(\d{1,2}):(\d{1,2})(?:\.(\d{1,}))?.*?$";

/// Minimum year for UTC time calculation (Sol 0).
pub const MIN_YEAR: i32 = 1873;

/// Minimum month for UTC time calculation (Sol 0).
pub const MIN_MONTH: u8 = 12;

/// Minimum day for UTC time calculation (Sol 0).
pub const MIN_DAY: u8 = 29;
