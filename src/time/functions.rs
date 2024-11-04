use hifitime::{ Epoch };
use std::f64::consts::PI;
use crate::time::constants::{ JD_ON_SOL_ZERO, SOL_IN_EARTH_DAYS, TAI_TO_TT_OFFSET };
use crate::time::structs::*;

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
/// This function may return the following errors:
///
/// - `TimeError::UtcTimeUnavailable`: If the current UTC time cannot be retrieved.
/// - `TimeError::LeapSecondError`: If the leap second information is unavailable.
/// - `TimeError::CalculationError`: If the calculated MSD is invalid or out of expected bounds.
///
/// # Advanced Usage
///
/// To get the current Sol with advanced calculations, use `msd_now(true)`.
/// Nasa tools use the simplified calculation for the current Sol, skipping the
/// leap second handling and the conversion to Barycentric Dynamical Time (TDB), with
/// which we should end up with more accurate results.

pub fn msd_now(advanced: bool) -> Result<f64, TimeError> {
    //  Get the current time in UTC
    let now_utc = get_current_utc_time().ok_or(TimeError::UtcTimeUnavailable)?;

    // If not advanced, simplify the calculations
    // MSD = (JD_TDB - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS
    if !advanced {
        let jde_tt = now_utc.to_jde_tt_days();

        let msd: f64 = (jde_tt - 2405522.0028779) / 1.0274912517;

        if msd.is_finite() && msd.is_sign_positive() {
            return Ok(msd);
        } else {
            return Err(TimeError::CalculationError);
        }
    }

    // Get official leap seconds
    let leap_seconds = now_utc.leap_seconds(true).ok_or(TimeError::LeapSecondError)?;

    // Convert UTC to TAI (International Atomic Time)
    // TAI = UTC + Leap Seconds
    let now_tai = now_utc + leap_seconds;

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
        Err(TimeError::CalculationError)
    }
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

        // Cusriosity Rover Landing sol
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
///     Ok(mtc) => println!("Mars Sol Date: {}", mtc),
///     Err(e) => eprintln!("Error calculating MTC: {}", e),
/// }
/// ```
///
/// # Errors
/// May propagate a `TimeError` if the `msd_now` function fails.
///
/// # Advanced Usage
///
/// To get the current mtc with advanced calculations, use `mtc_now(true)`.
/// Nasa tools use the simplified calculation for the current Sol (which is used to get current_mtc), skipping the
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

        // Cusriosity Rover Landing MTC - 5:53:28
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        assert!(mtc.hours == 5, "MTC hours are off for Curiosity mission Sol 0");

        assert!(mtc.minutes == 53, "MTC minutes are off for Curiosity mission Sol 0");

        assert!(mtc.seconds == 28, "MTC seconds are off for Curiosity mission Sol 0");
    }
}
