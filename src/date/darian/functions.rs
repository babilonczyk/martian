use crate::date::darian::constants::{ DARIAN_MONTH_LENGTHS, SOL_DIFFERENCE_BETWEEN_DARIAN_AND_MSD };
use crate::date::darian::errors::*;
use crate::date::darian::structs::*;

#[cfg(not(test))]
use crate::time::msd_now;

// ------------------------------------------------------------------------------------------------
/// Get current Darian Date on Mars.
///
/// # Examples
///
/// ```
/// use martian::date::darian_now;
///
/// let darian_date = darian_now().unwrap();
/// println!("Current Darian Date: {}", darian_date);
/// ```
///
/// # Errors
///
/// It may propagate errors from time modules:
///
/// - `DateError::TimeError(TimeError::<UtcTimeUnavailable>)`

pub fn darian_now() -> Result<DarianDate, DateError> {
    let msd = get_msd_now()?;
    Ok(msd_to_darian(msd)?)
}

#[cfg(not(test))]
fn get_msd_now() -> Result<f64, DateError> {
    Ok(msd_now()?)
}

#[cfg(test)]
fn get_msd_now() -> Result<f64, DateError> {
    // Sol for 2024-11-07T17:58:40.000
    Ok(53626.0011)
}

#[cfg(test)]
mod darian_now_tests {
    use super::*;

    #[test]
    fn test_darian_now() {
        let darian_date = darian_now().unwrap();
        let expected_darian_date = DarianDate::new(220, 24, 25.0);

        let year = darian_date.year;
        let month = darian_date.month;
        let sol = darian_date.sol;

        assert!(
            year == expected_darian_date.year,
            "Year: {} != {}",
            year,
            expected_darian_date.year
        );

        assert!(
            month == expected_darian_date.month,
            "Month: {} != {}",
            month,
            expected_darian_date.month
        );

        assert!(
            (sol - expected_darian_date.sol).abs() < 0.1,
            "Sol: {} != {}",
            sol,
            expected_darian_date.sol
        );
    }
}

// ------------------------------------------------------------------------------------------------
/// Converts a given MSD to a Darian date.
///
/// # Arguments
///
/// * `msd` -  Martian Sol Date to be converted to Darian date.
///
/// # Examples
///
/// ```
/// use martian::date::msd_to_darian;
///
/// let msd = 53626.0011;
///
/// let darian_date = msd_to_darian(msd).unwrap();
/// println!("Darian Date: {}", darian_date);
/// ```
///

pub fn msd_to_darian(msd: f64) -> Result<DarianDate, DateError> {
    // Adjust the MSD to the Darian calendar starting point
    // Martian Sol Date starts with sol 0 on 1873-12-29 12:09 UTC
    // While Darian calendar starts with sol 1 on 1609-03-01 18:40:34 UTC
    let adjusted_msd = msd + SOL_DIFFERENCE_BETWEEN_DARIAN_AND_MSD - 1.0;

    // Split into total sols and fractional part
    let total_sols = adjusted_msd.floor() as u32;
    let sol_fraction = adjusted_msd - adjusted_msd.floor();

    let mut sols_remaining = total_sols;
    let mut year = 0;

    // Determine the year
    loop {
        let year_length = if is_darian_leap_year(year) { 669 } else { 668 };
        if sols_remaining >= year_length {
            sols_remaining -= year_length;
            year += 1;
        } else {
            break;
        }
    }

    // Determine the month and sol
    let mut month = 1;
    let sol;
    loop {
        let month_length = get_darian_month_length(year, month)? as u32;
        if sols_remaining < month_length {
            sol = (sols_remaining as f64) + sol_fraction;
            break;
        } else {
            sols_remaining -= month_length;
            month += 1;
        }
    }

    Ok(DarianDate::new(year, month, sol))
}

// Determines if a given Martian year is a leap year in the Darian calendar
fn is_darian_leap_year(year: i32) -> bool {
    if year % 100 == 0 {
        if year % 500 == 0 { true } else { false }
    } else {
        year % 2 != 0 || year % 10 == 0
    }
}

// Returns the length of a given month in a specific Martian year
fn get_darian_month_length(year: i32, month: u8) -> Result<u8, DateError> {
    if month < 1 || month > 24 {
        return Err(DateError::MonthValueOutOfRange);
    }

    let base_length = DARIAN_MONTH_LENGTHS[(month - 1) as usize];
    Ok(if is_darian_leap_year(year) && month == 24 { base_length + 1 } else { base_length })
}

#[cfg(test)]
mod msd_to_darian_tests {
    use super::*;

    #[test]
    fn test_msd_to_darian_date() {
        // "2024-11-07T17:58:40.000";
        let msd = 53626.0011;
        let expected_darian_date = DarianDate::new(220, 24, 25.0);

        let result = msd_to_darian(msd).unwrap();

        let year = result.year;
        let month = result.month;
        let sol = result.sol;

        assert!(
            year == expected_darian_date.year,
            "Year: {} != {}",
            year,
            expected_darian_date.year
        );
        assert!(
            month == expected_darian_date.month,
            "Month: {} != {}",
            month,
            expected_darian_date.month
        );
        assert!(
            (sol - expected_darian_date.sol).abs() < 0.1,
            "Sol: {} != {}",
            sol,
            expected_darian_date.sol
        );
    }
}
