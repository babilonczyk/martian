/// Regex pattern for ISO8601 date format used in the library (YYYY-MM-DDTHH:MM:SS.sss).
pub const ISO8601_REGEX: &str =
    r"^(\d{1,})-(\d{1,2})-(\d{1,2})T(\d{1,2}):(\d{1,2}):(\d{1,2})(?:\.(\d{1,}))?.*?$";
