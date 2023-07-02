use std::f64::consts::TAU;
use std::time::SystemTime;

mod consts;
pub use consts::*;

fn julian_date(time: SystemTime) -> f64 {
    let unix_time: f64 = match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(earlier ) => -1. * earlier.duration().as_secs_f64(),
    };
    unix_time / 86400. + 2440587.5
}
fn illumination(phase: f64) -> f64 {
    0.5 * (1.0 - (TAU * phase).cos())
}
fn lunation(julian_date: f64) -> u16 {
    (1. + (julian_date - LUNATION_BASE) / ORBIT_PERIOD).floor() as u16
}

fn phase(julian_date: f64) -> f64 {
    ((julian_date - ORBIT_OFFSET) / ORBIT_PERIOD).fract()
}
fn distance_phase(julian_date: f64) -> f64 {
    ((julian_date - DISTANCE_OFFSET) / DISTANCE_PERIOD).fract()
}
fn distance(phase:f64, julian_date: f64) -> f64 {

    let distance_p = distance_phase(julian_date);
    let distance_p_tau: f64 = TAU * distance_p;
    let p_tau: f64 = 2.0 * TAU * phase;
    let p_distance_tau_diff = p_tau - distance_p_tau;

    60.4 - 3.3 * distance_p_tau.cos() - 0.6 * (p_distance_tau_diff).cos() - 0.5 * (p_tau).cos()
}
impl Moon {
    pub fn new(time: SystemTime) -> Moon {
        let julian_date: f64 = julian_date(time);
        let phase: f64 = phase(julian_date);
        let age: f64 = phase * ORBIT_PERIOD;
        let illumination: f64 = illumination(phase);
        let lunation: u16 = lunation(julian_date);
        let distance: f64 = distance(phase, julian_date);
        
        return Moon {
            julian_date,
            age,
            phase,
            illumination,
            distance,
            lunation,
        };
    }
    pub fn distance_km(&self) -> f64 {
        self.distance * EARTH_RADIUS_KM
    }
    pub fn is_waning(&self)-> bool{
        self.age < 0.5
    }
    pub fn is_waxing(&self)-> bool{
        self.age > 0.5
    }
    pub fn phase_name(&self, ) -> &'static str {
        for phase in PHASES.iter() {
            if self.phase >= phase.start && self.phase < phase.end {
                return phase.name;
            }
        }
        return "Unknown";
    }
    pub fn phase_emoji(&self, ) -> &'static str {
        for phase in PHASES.iter() {
            if self.phase >= phase.start && self.phase < phase.end {
                return phase.emoji;
            }
        }
        return "Unknown";
    }
    
}