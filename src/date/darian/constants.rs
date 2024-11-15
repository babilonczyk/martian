/// Darian month lengths in sols, starting from month 1 to month 24
pub const DARIAN_MONTH_LENGTHS: [u8; 24] = [
    28, // Sagittarius
    28, // Dhanus
    28, // Capricornus
    28, // Makara
    28, // Aquarius
    27, // Kumbha
    28, // Pisces
    28, // Mina
    28, // Aries
    28, // Mesha
    28, // Taurus
    27, // Rishabha
    28, // Gemini
    28, // Mithuna
    28, // Cancer
    28, // Karka
    28, // Leo
    27, // Simha
    28, // Virgo
    28, // Kanya
    28, // Libra
    28, // Tula
    28, // Scorpius
    27, // Vrishchika
];

/// Darian month names
pub const DARIAN_MONTH_NAMES: [&str; 24] = [
    "Sagittarius",
    "Dhanus",
    "Capricornus",
    "Makara",
    "Aquarius",
    "Kumbha",
    "Pisces",
    "Mina",
    "Aries",
    "Mesha",
    "Taurus",
    "Rishabha",
    "Gemini",
    "Mithuna",
    "Cancer",
    "Karka",
    "Leo",
    "Simha",
    "Virgo",
    "Kanya",
    "Libra",
    "Tula",
    "Scorpius",
    "Vrishchika",
];

/// Total number of sols in a non-leap year.
pub const DARIAN_YEAR_SOLS: u16 = 668;

/// Difference of sols between starting points of Darian Calendar and MSD.
pub const SOL_DIFFERENCE_BETWEEN_DARIAN_AND_MSD: f64 = 94130.9446045;
