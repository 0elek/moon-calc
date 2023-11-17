/// The period of the lunar orbit in days.
pub const ORBIT_PERIOD: f64 = 29.53058770576;

/// The offset for the lunar orbit calculations.
pub const ORBIT_OFFSET: f64 = 2451550.26;

/// The period of the lunar distance in days.
pub const DISTANCE_PERIOD: f64 = 27.55454988;

/// The offset for the lunar distance calculations.
pub const DISTANCE_OFFSET: f64 = 2451562.2;

/// The base value for calculating lunation.
pub const LUNATION_BASE: f64 = 2423436.6115277777;

/// The mean radius of the Earth in kilometers.
pub const EARTH_RADIUS_KM: f64 = 6371.0084;


/// Represents a lunar phase with name, emoji and start and end fractions.
#[derive(Debug, Clone, Copy)]
pub struct Phase {
    /// Name of the lunar phase.
    pub name: &'static str,
    /// Emoji representing the lunar phase.
    pub emoji: &'static str,
    /// Start fraction of the lunar phase.
    pub start: f64,
    /// End fraction of the lunar phase.
    pub end: f64,
}
pub const PHASES: [Phase; 8] = [
    Phase { emoji: "🌑", name: "New Moon", start: 0.0, end: 0.02 },
    Phase { emoji: "🌒", name: "Waxing Crescent", start: 0.02, end: 0.22 }, 
    Phase { emoji: "🌓", name: "First Quarter", start: 0.22, end: 0.27 },
    Phase { emoji: "🌔", name: "Waxing Gibbous", start: 0.27, end: 0.47 },
    Phase { emoji: "🌕", name: "Full Moon", start: 0.47, end: 0.52 },
    Phase { emoji: "🌖", name: "Waning Gibbous", start: 0.52, end: 0.72 },
    Phase { emoji: "🌗", name: "Last Quarter", start: 0.72, end: 0.77 },
    Phase { emoji: "🌘", name: "Waning Crescent", start: 0.77, end: 1.0 },
];

/// Represents information about the moon, including its julian date, phase,
/// age, illumination, distance, and lunation.
#[derive(Debug, Clone, Copy)]
pub struct Moon {
    /// A continuous count of days and fractions since noon Universal Time on January 1, 4713 BC 
    pub julian_date: f64,
    /// Phase of the moon.
    pub phase: f64,
    /// Age of the moon.
    pub age: f64,
    /// Illumination of the moon (0 to 1 where 1 is a full moon).
    pub illumination: f64,
    /// Distance of the moon in earth radii.
    pub distance: f64,
    /// Lunation number.
    pub lunation: u16,
}

