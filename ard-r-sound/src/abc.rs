use std::collections::HashMap;

pub use ard_r_sound_base::*;

/// Everything in an ABC file
#[derive(Debug, Clone, Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct ABC {
    pub version: Option<Version>,
    pub headers: Headers,
    pub notes: Vec<Note>,
}

impl ABC {
    pub fn total_playtime_secs(&self) -> f64 {
        let mut total = 0f64;
        // TODO: support non 4/4 time
        const BASE: f64 = 1. / 4.;
        for note in &self.notes {
            total += match note.length {
                Length::Unit => BASE,
                Length::Multiple(x) => BASE * x as f64,
                Length::Division(x) => BASE / x as f64,
            }
        }
        total
    }
}

/// The version of an ABC file
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub type Headers = HashMap<char, Vec<String>>;
