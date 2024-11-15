use std::fmt;

// ------------------------------------------------------------------------------------------------
/// Represents a date with year, month and sol value based on the Darian calendar.
#[derive(Debug, PartialEq)]
pub struct DarianDate {
    pub year: i32,
    pub month: u8,
    pub sol: f64,
}

impl DarianDate {
    pub fn new(year: i32, month: u8, sol: f64) -> Self {
        Self { year, month, sol }
    }
}

impl fmt::Display for DarianDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sol_int = self.sol.floor() as u8;
        let sol_frac = self.sol - (sol_int as f64);

        if sol_frac == 0.0 {
            write!(f, "{}-{}-{:.5}", self.year, self.month, sol_int)
        } else {
            write!(f, "{}-{}-{:.5}", self.year, self.month, sol_int)
        }
    }
}
