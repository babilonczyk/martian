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
/// match msd_now() {
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

pub fn msd_now() -> Result<f64, TimeError> {
    //  Get the current time in UTC
    let now_utc = get_current_utc_time().ok_or(TimeError::UtcTimeUnavailable)?;

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

    // MSD = (JD_TDB - JD_MSD0) / SOL_IN_EARTH_DAYS
    let msd = (jd_tdb - JD_ON_SOL_ZERO) / SOL_IN_EARTH_DAYS;

    if msd.is_finite() && msd.is_sign_positive() {
        Ok(msd)
    } else {
        Err(TimeError::CalculationError)
    }
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
mod tests {
    use super::*;

    #[test]
    fn test_curiosity_mission_sol() {
        let result = msd_now().unwrap();

        // Cusriosity Rover Landing sol
        // Value taken from https://www.giss.nasa.gov/tools/mars24/
        let curiosity_landing_sol = 49269.25;

        assert!(
            (result - curiosity_landing_sol).abs() < 0.01,
            "MSD calculation is off for Curiosity mission Sol 0"
        );
    }
}
