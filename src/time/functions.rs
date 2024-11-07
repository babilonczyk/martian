use hifitime::{ Epoch };
use std::f64::consts::PI;
use crate::time::structs::*;
use regex::Regex;
use crate::time::constants::{
    JD_ON_SOL_ZERO,
    SOL_IN_EARTH_DAYS,
    TAI_TO_TT_OFFSET,
    ISO8601_REGEX,
    MIN_YEAR,
    MIN_MONTH,
    MIN_DAY,
};

// ------------------------------------------------------------------------------------------------
/// Get current Sol (MSD) on Mars.
///
/// # Examples
///
/// ```
/// use martian::time::msd_now;
///
/// let advanced = false;
/// match msd_now(advanced) {
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
/// - `TimeError::LeapSecondError`: If the leap second information is unavailable.
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.
///
/// # Advanced Usage
///
/// To get the current Sol with advanced calculations, use `msd_now(true)`.
/// Nasa tool like Mars24 use the simplified calculation for the current Sol, skipping the
/// leap second handling and the conversion to Barycentric Dynamical Time (TDB), with
/// which we should end up with more accurate results.

pub fn msd_now(advanced: bool) -> Result<f64, TimeError> {
    //  Get the current time in UTC
    let utc_epoch = get_current_utc_time().ok_or(TimeError::UtcTimeUnavailable)?;

    // Convert the UTC Epoch to a ISO8601 string
    let iso = utc_epoch.to_isoformat();

    // Convert UTC to MSD
    let msd = utc_to_msd(&iso, advanced)?;

    Ok(msd)
}

/// Alias for `msd_now()`.
pub fn current_sol(advanced: bool) -> Result<f64, TimeError> {
    let msd = msd_now(advanced);
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
        let result_basic = msd_now(false).unwrap();
        let result_advanced = msd_now(true).unwrap();

        // Curiosity Rover Landing sol
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        let curiosity_landing_sol = 49269.25;

        assert!(
            (result_basic - curiosity_landing_sol).abs() < 0.01,
            "MSD calculation is off for Curiosity mission Sol 0"
        );

        assert!(
            (result_advanced - curiosity_landing_sol).abs() < 0.01,
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
/// let advanced = false;
/// let date_time = "2012-08-06T05:17:57.000";
///
/// match utc_to_msd(&date_time, advanced) {
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
/// - `TimeError::UtcTimeUnavailable`: If the current UTC time cannot be retrieved.
/// - `TimeError::LeapSecondError`: If the leap second information is unavailable.
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.
///
/// # Advanced Usage
///
/// To get the Sol with advanced calculations, use `utc_to_msd(date_time, true)`.
/// Nasa tool like Mars24 uses the simplified calculation for getting the Sol value, skipping the
/// leap second handling and the conversion to Barycentric Dynamical Time (TDB), with
/// which we should end up with more accurate results.

pub fn utc_to_msd(datetime: &str, advanced: bool) -> Result<f64, TimeError> {
    let regex = Regex::new(ISO8601_REGEX).map_err(|_| TimeError::ISO8601FormatError)?;
    let regex_result = regex.captures(datetime).ok_or(TimeError::ISO8601FormatError)?;

    let year = validate_regex_value(regex_result.get(1), 0, i32::MAX)?; // Didn't set min to MIN_YEAR to get more meaningfull error when validating whole YYYY-MM-DD
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

    // If not advanced, simplify the calculations
    // MSD = (JD_TDB - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS
    if !advanced {
        let jde_tt = utc.to_jde_tt_days();

        let msd: f64 = (jde_tt - 2405522.0028779) / 1.0274912517;

        if msd.is_finite() && msd.is_sign_positive() {
            return Ok(msd);
        } else {
            return Err(TimeError::TimeCalculationError);
        }
    }

    // Get official leap seconds
    let leap_seconds = utc.leap_seconds(true).ok_or(TimeError::LeapSecondError)?;

    // Convert UTC to TAI (International Atomic Time)
    // TAI = UTC + Leap Seconds
    let now_tai = utc + leap_seconds;

    // Convert TAI to TT (Terrestrial Time)
    // TT is exactly 32.184 seconds ahead of TAI.
    let now_tt = now_tai + TAI_TO_TT_OFFSET;

    // JD_TT Julian Date in TT
    let jd_tt = now_tt.to_jde_tt_days();

    // Convert JD_TT to JD_TDB (Barycentric Dynamical Time)
    // JD_TDB ≈ JD_TT + 0.001658 * sin(M) + 0.000014 * sin(2 * M)
    // where M = 6.24004077 + 0.01720197034 * (JD_TT - 2451545.0)
    let m = (6.24004077 + 0.01720197034 * (jd_tt - 2451545.0)) % (2.0 * PI);
    let jd_tdb = jd_tt + 0.001658 * m.sin() + 0.000014 * (2.0 * m).sin();

    // MSD = (JD_TDB - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS
    let msd = (jd_tdb - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS;

    if msd.is_finite() && msd.is_sign_positive() {
        Ok(msd)
    } else {
        Err(TimeError::TimeCalculationError)
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
    if year < MIN_YEAR {
        Err(TimeError::DateBelowSolZeroError)
    } else if year == MIN_YEAR {
        if month < MIN_MONTH {
            Err(TimeError::DateBelowSolZeroError)
        } else if month == MIN_MONTH && day < MIN_DAY {
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

        let result_basic = utc_to_msd(date_time, false).unwrap();
        let result_advanced = utc_to_msd(date_time, true).unwrap();

        assert!(
            (result_basic - curiosity_landing_sol).abs() < 0.01,
            "UTC to MSD calculation is off for Curiosity mission Sol 0 by {:.7}",
            result_basic - curiosity_landing_sol
        );

        assert!(
            (result_advanced - curiosity_landing_sol).abs() < 0.01,
            "UTC to MSD calculation is off for Curiosity mission Sol 0 by {:.7}",
            result_advanced - curiosity_landing_sol
        );

        // Test on more accurate value
        let date_time = "2024-11-07T17:58:40.000";
        let msd = 53626.0011;

        let result = utc_to_msd(date_time, false).unwrap();

        assert!(
            (result - msd).abs() < 0.00001,
            "UTC to MSD calculation is off for Curiosity mission Sol 0 by {:.7}",
            result - msd
        );
    }

    #[test]
    fn test_utc_to_msd_invalid_date_format() {
        let advanced = false;

        let date_time = "21-08-06T05:17:57.000";
        let result = utc_to_msd(date_time, advanced);

        assert_eq!(result.unwrap_err(), TimeError::ISO8601FormatError);
    }

    #[test]
    fn test_utc_to_msd_before_sol_zero() {
        let advanced = false;

        let date_time = "1873-12-28T23:59:59.999";
        let result = utc_to_msd(date_time, advanced);

        assert_eq!(result.unwrap_err(), TimeError::DateBelowSolZeroError);
    }

    #[test]
    fn test_utc_to_msd_invalid_date() {
        let advanced = false;

        // Set to invalid date
        let date_time = "2021-13-29T00:00:00.000";
        let result = utc_to_msd(date_time, advanced);

        assert_eq!(result.unwrap_err(), TimeError::InvalidArgumentError);
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
/// let advanced = false;
/// match mtc_now(advanced) {
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
/// - `TimeError::LeapSecondError`: If the leap second information is unavailable.
/// - `TimeError::TimeCalculationError`: If the calculated MSD is invalid or out of expected bounds.
///
/// # Advanced Usage
///
/// To get the current mtc with advanced calculations, use `mtc_now(true)`.
/// Nasa tool like Mars24 use the simplified calculation for the current Sol (which is used to get current_mtc), skipping the
/// leap second handling and the conversion to Barycentric Dynamical Time (TDB), with
/// which we should end up with more accurate results.

pub fn mtc_now(advanced: bool) -> Result<Time, TimeError> {
    // Get current sol
    let msd = msd_now(advanced)?;

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
        let mtc = mtc_now(false).unwrap();

        // Curiosity Rover Landing MTC - 5:53:28
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        assert!(mtc.hours == 5, "MTC hours are off for Curiosity mission Sol 0");
        assert!(mtc.minutes == 53, "MTC minutes are off for Curiosity mission Sol 0");
        assert!(mtc.seconds == 28, "MTC seconds are off for Curiosity mission Sol 0");
    }
}
