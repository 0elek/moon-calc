pub const ORBIT_PERIOD: f64 = 29.53058770576;
pub const ORBIT_OFFSET: f64 = 2451550.26;

pub const DISTANCE_PERIOD: f64 = 27.55454988;
pub const DISTANCE_OFFSET: f64 = 2451562.2;

pub const LUNATION_BASE: f64 = 2423436.6115277777;

pub const EARTH_RADIUS_KM: f64 = 6371.0084;


pub struct Phase {
    pub emoji: &'static str,
    pub name: &'static str,
    pub start: f64,
    pub end: f64,
}
pub const PHASES: [Phase; 8] = [
    Phase {
        emoji: "🌑",
        name: "New Moon",
        start: 0.0,
        end: 0.02,
    },
    Phase {
        emoji: "🌒",
        name: "Waxing Crescent",
        start: 0.02,
        end: 0.22,
    }, 
    Phase {
        emoji: "🌓",
        name: "First Quarter",
        start: 0.22,
        end: 0.27,
    },
    Phase {
        emoji: "🌔",
        name: "Waxing Gibbous",
        start: 0.27,
        end: 0.47,
    },
    Phase {
        emoji: "🌕",
        name: "Full Moon",
        start: 0.47,
        end: 0.52,
    },
    Phase {
        emoji: "🌖",
        name: "Waning Gibbous",
        start: 0.52,
        end: 0.72,
    },
    Phase {
        emoji: "🌗",
        name: "Last Quarter",
        start: 0.72,
        end: 0.77,
    },
    Phase {
        emoji: "🌘",
        name: "Waning Crescent",
        start: 0.77,
        end: 1.0,
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Moon {
    pub julian_date: f64,
    pub phase: f64,
    pub age: f64, 
    pub illumination: f64, // 0 to 1 where is 1 is full moon
    pub distance: f64, // in earth radii
    pub lunation: u16, // brown l
}