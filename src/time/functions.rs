use hifitime::{ Epoch, Duration };
use crate::time::structs::*;
use crate::time::errors::*;
use regex::Regex;
use crate::time::constants::{
    JD_ON_SOL_ZERO,
    SOL_IN_EARTH_DAYS,
    ISO8601_REGEX,
    MIN_SOL_YEAR,
    MIN_SOL_MONTH,
    MIN_SOL_DAY,
};

// ------------------------------------------------------------------------------------------------
/// Get current Sol (MSD) on Mars.
///
/// # Examples
///
/// ```
/// use martian::time::msd_now;
///
/// match msd_now() {
///     Ok(msd) => println!("Mars Sol Date: {:.7}", msd),
///     Err(e) => eprintln!("Error calculating MSD: {}", e),
/// }
/// ```
///
/// # Errors
///
/// May propagate Errors from `utc_to_msd` if function fails.
///
/// - `TimeError::ISO8601FormatError`: If the provided date does not match the ISO8601 format.
/// - `TimeError::DateBelowSolZeroError`: If the provided date is below Sol 0 (1873-12-29T00:00:00.000 UTC).
/// - `TimeError::InvalidArgumentError`: If the provided argument value is invalid (didn't pass validation).
/// - `TimeError::UtcTimeUnavailable`: If the current UTC time cannot be retrieved.
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.

pub fn msd_now() -> Result<f64, TimeError> {
    //  Get the current time in UTC
    let utc_epoch = get_current_utc_time().ok_or(TimeError::UtcTimeUnavailable)?;

    // Convert the UTC Epoch to a ISO8601 string
    let iso = utc_epoch.to_isoformat();

    // Convert UTC to MSD
    let msd = utc_to_msd(&iso)?;

    Ok(msd)
}

/// Alias for `msd_now()`.
pub fn current_sol() -> Result<f64, TimeError> {
    let msd = msd_now();
    msd
}

#[cfg(not(test))]
fn get_current_utc_time() -> Option<Epoch> {
    Epoch::now().ok()
}

#[cfg(test)]
fn get_current_utc_time() -> Option<Epoch> {
    // Curiosity Rover landing time
    let time = Epoch::from_gregorian_utc(2012, 8, 6, 5, 17, 57, 0);
    Some(time)
}

#[cfg(test)]
mod msd_now_tests {
    use super::*;

    #[test]
    fn test_curiosity_mission_sol() {
        let result = msd_now().unwrap();

        // Curiosity Rover Landing sol
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        let curiosity_landing_sol = 49269.25;

        assert!(
            (result - curiosity_landing_sol).abs() < 0.01,
            "MSD calculation is off for Curiosity mission Sol 0"
        );
    }
}

// ------------------------------------------------------------------------------------------------
/// Convert UTC datetime to the Sol Date (MSD) on Mars. Requires an ISO8601 formatted datetime string as input.
///
/// # Examples
///
/// ```
/// use martian::time::utc_to_msd;
///
/// let date_time = "2012-08-06T05:17:57.000";
///
/// match utc_to_msd(&date_time) {
///     Ok(msd) => println!("Mars Sol Date: {:.7}", msd),
///     Err(e) => eprintln!("Error calculating MSD: {}", e),
/// }
/// ```
///
/// # Errors
///
/// This function may return the following errors:
///
/// - `TimeError::ISO8601FormatError`: If the provided date does not match the ISO8601 format.
/// - `TimeError::DateBelowSolZeroError`: If the provided date is below Sol 0 (1873-12-29T00:00:00.000 UTC).
/// - `TimeError::InvalidArgumentError`: If the provided argument value is invalid (didn't pass validation).
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.

pub fn utc_to_msd(datetime: &str) -> Result<f64, TimeError> {
    let regex = Regex::new(ISO8601_REGEX).map_err(|_| TimeError::ISO8601FormatError)?;
    let regex_result = regex.captures(datetime).ok_or(TimeError::ISO8601FormatError)?;

    let year = validate_regex_value(regex_result.get(1), 0, i32::MAX)?; // Didn't set min to MIN_SOL_YEAR to get more meaningfull error when validating whole YYYY-MM-DD
    let month = validate_regex_value(regex_result.get(2), 1, 12)?;
    let day = validate_regex_value(regex_result.get(3), 1, 31)?; // TODO: Validate days per month

    // YYYY-MM-DD must be Sol 0 or later
    validate_date(year, month, day)?;

    let hour = validate_regex_value(regex_result.get(4), 0, 23)?;
    let minute = validate_regex_value(regex_result.get(5), 0, 59)?;
    let second = validate_regex_value(regex_result.get(6), 0, 59)?;

    // If milliseconds are not present, default to 0
    let millisecond = regex_result.get(7).map_or(0, |m| m.as_str().parse::<u32>().unwrap_or(0));

    // Convert the provided date and time to an UTC Epoch
    let utc = Epoch::from_gregorian_utc(year, month, day, hour, minute, second, millisecond);

    // MSD = (JD_TDB - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS
    let jde_tt = utc.to_jde_tt_days();

    let msd: f64 = (jde_tt - 2405522.0028779) / 1.0274912517;

    if msd.is_finite() && msd.is_sign_positive() {
        return Ok(msd);
    } else {
        return Err(TimeError::TimeCalculationError);
    }
}

fn validate_regex_value<T: std::str::FromStr>(
    input: Option<regex::Match>,
    min: T,
    max: T
) -> Result<T, TimeError>
    where T: PartialOrd
{
    let value = input
        .ok_or(TimeError::InvalidArgumentError)?
        .as_str()
        .parse::<T>()
        .map_err(|_| TimeError::InvalidArgumentError)?;
    if value >= min && value <= max {
        Ok(value)
    } else {
        Err(TimeError::InvalidArgumentError)
    }
}

fn validate_date(year: i32, month: u8, day: u8) -> Result<(), TimeError> {
    if year < MIN_SOL_YEAR {
        Err(TimeError::DateBelowSolZeroError)
    } else if year == MIN_SOL_YEAR {
        if month < MIN_SOL_MONTH {
            Err(TimeError::DateBelowSolZeroError)
        } else if month == MIN_SOL_MONTH && day < MIN_SOL_DAY {
            Err(TimeError::DateBelowSolZeroError)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod utc_to_msd_tests {
    use super::*;

    #[test]
    fn test_utc_to_msd_success() {
        // Curiosity Rover Landing  2012-08-06T05:17:57.000
        // Values taken from https://www.giss.nasa.gov/tools/mars24/
        let date_time = "2012-08-06T05:17:57.000";
        let curiosity_landing_sol = 49269.25;

        let result = utc_to_msd(date_time).unwrap();

        assert!(
            (result - curiosity_landing_sol).abs() < 0.01,
            "UTC to MSD calculation is off for Curiosity mission Sol 0 by {:.7}",
            result - curiosity_landing_sol
        );

        // Test on more accurate value
        let date_time = "2024-11-07T17:58:40.000";
        let msd = 53626.0011;

        let result = utc_to_msd(date_time).unwrap();

        assert!(
            (result - msd).abs() < 0.00001,
            "UTC to MSD calculation is off for Curiosity mission Sol 0 by {:.7}",
            result - msd
        );
    }

    #[test]
    fn test_utc_to_msd_invalid_date_format() {
        let date_time = "21-08-06T05:17:57.000";
        let result = utc_to_msd(date_time);

        assert_eq!(result.unwrap_err(), TimeError::ISO8601FormatError);
    }

    #[test]
    fn test_utc_to_msd_before_sol_zero() {
        let date_time = "1873-12-28T23:59:59.999";
        let result = utc_to_msd(date_time);

        assert_eq!(result.unwrap_err(), TimeError::DateBelowSolZeroError);
    }

    #[test]
    fn test_utc_to_msd_invalid_date() {
        // Set to invalid date
        let date_time = "2021-13-29T00:00:00.000";
        let result = utc_to_msd(date_time);

        assert_eq!(result.unwrap_err(), TimeError::InvalidArgumentError);
    }
}

// ------------------------------------------------------------------------------------------------
/// Convert Mars Sol Date (MSD) to UTC datetime. Returns an ISO8601 formatted datetime string.
///
/// # Examples
///
/// ```
/// use martian::time::msd_to_utc;
///
/// let msd = 49269.25;
///
/// match msd_to_utc(msd) {
///     Ok(utc) => println!("UTC: {}", utc),
///     Err(e) => eprintln!("Error calculating UTC: {}", e),
/// }
/// ```
///
/// # Errors
///
/// This function may return the following errors:
///
/// - `TimeError::DateBelowSolZeroError`: If the provided date is below Sol 0 (1873-12-29T00:00:00.000 UTC).
/// - `TimeError::LeapSecondError`: If the leap second information is unavailable.

pub fn msd_to_utc(msd: f64) -> Result<String, TimeError> {
    if msd < 0.0 {
        return Err(TimeError::DateBelowSolZeroError);
    }

    let jd_tdb = msd * SOL_IN_EARTH_DAYS + JD_ON_SOL_ZERO;

    let tt_epoch = Epoch::from_jde_tdb(jd_tdb);

    let leap_seconds = tt_epoch.leap_seconds(true).ok_or(TimeError::LeapSecondError)?;
    let utc_epoch = tt_epoch - Duration::from_seconds(leap_seconds);

    return Ok(utc_epoch.to_isoformat());
}

#[cfg(test)]
mod msd_to_utc_tests {
    use super::*;

    #[test]
    fn test_utc_to_msd_success() {
        let date_time = "2024-11-07T17:58:40.000";
        let msd = 53626.0011;
        let result = msd_to_utc(msd).unwrap();

        let expected_epoch = Epoch::from_gregorian_str(date_time).unwrap();
        let result_epoch = Epoch::from_gregorian_str(&result).unwrap();

        let difference = (expected_epoch - result_epoch).abs().to_seconds();

        assert!(difference <= 1.0, "Difference is more than 1 second: {} seconds", difference);
    }

    #[test]
    fn utc_to_msd_and_back() {
        let date_time = "2024-11-07T17:58:40.000";

        let msd = utc_to_msd(date_time).unwrap();

        let result = msd_to_utc(msd).unwrap();

        let expected_epoch = Epoch::from_gregorian_str(date_time).unwrap();
        let result_epoch = Epoch::from_gregorian_str(&result).unwrap();

        let difference = (expected_epoch - result_epoch).abs().to_seconds();

        assert!(difference <= 1.0, "Difference is more than 1 second: {} seconds", difference);
    }
}

// ------------------------------------------------------------------------------------------------
/// Get current Martian Coordinated Time (MTC) on Mars.
///
/// # Returns
/// * `Result<Time, TimeError>` - The MTC time as a `Time` struct on a 24-hour Martian clock
///
/// ```
/// use martian::time::mtc_now;
///
/// match mtc_now() {
///     Ok(mtc) => println!("Mars Coordinated Time: {}", mtc),
///     Err(e) => eprintln!("Error calculating MTC: {}", e),
/// }
/// ```
///
/// # Errors
///
/// May propagate Errors from `msd_now` if function fails.
///
/// - `TimeError::ISO8601FormatError`: If the provided date does not match the ISO8601 format.
/// - `TimeError::DateBelowSolZeroError`: If the provided date is below Sol 0 (1873-12-29T00:00:00.000 UTC).
/// - `TimeError::InvalidArgumentError`: If the provided argument value is invalid (didn't pass validation).
/// - `TimeError::UtcTimeUnavailable`: If the current UTC time cannot be retrieved.
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.

pub fn mtc_now() -> Result<Time, TimeError> {
    // Get current sol
    let msd = msd_now()?;

    // MTC = (24 * MSD) % 24
    let mtc_hours = (24.0 * msd) % 24.0;

    // Extract hours, minutes, seconds, and milliseconds
    let hours = mtc_hours;
    let minutes = (mtc_hours % 1.0) * 60.0;
    let seconds = ((mtc_hours * 60.0) % 1.0) * 60.0;
    let milliseconds = (seconds % 1.0) * 1000.0;

    Ok(
        Time::new(
            hours.floor() as u32,
            minutes.floor() as u32,
            seconds.floor() as u32,
            milliseconds.round() as u32
        )
    )
}

#[cfg(test)]
mod mtc_now_tests {
    use super::*;

    #[test]
    fn test_mtc() {
        let mtc = mtc_now().unwrap();

        // Curiosity Rover Landing MTC - 5:53:28
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        assert!(mtc.hours == 5, "MTC hours are off for Curiosity mission Sol 0");
        assert!(mtc.minutes == 53, "MTC minutes are off for Curiosity mission Sol 0");
        assert!(mtc.seconds == 28, "MTC seconds are off for Curiosity mission Sol 0");
    }
}
