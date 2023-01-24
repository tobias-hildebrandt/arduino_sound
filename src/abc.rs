use std::collections::HashMap;

use enum_iterator::Sequence;

/// Everything in an ABC file
#[derive(Debug, Clone)]
pub struct ABC {
    pub version: Option<Version>,
    pub headers: Headers,
    pub notes: Vec<Note>,
}

impl Default for ABC {
    fn default() -> Self {
        Self {
            version: None,
            headers: Headers::new(),
            notes: Vec::new(),
        }
    }
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

/// A single note
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Note {
    pub pitch: PitchOrRest,
    pub length: Length,
}

/// Musical pitch or a rest
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PitchOrRest {
    Pitch {
        class: PitchClass,
        /// Octaves above or below the base octave
        octave: i8,
    },
    Rest,
}

/// Twelve-tone pitch class
/// TODO: is this relative to key?
#[derive(Debug, Copy, Clone, Sequence, PartialEq, Eq, Hash)]
pub enum PitchClass {
    A,
    ASharpBFlat,
    B,
    C,
    CSharpDFlat,
    D,
    DSharpEFlat,
    E,
    F,
    FSharpGFlat,
    G,
    GSharpAFlat,
}

impl TryFrom<char> for PitchClass {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        return Ok(match value.to_ascii_lowercase() {
            'a' => PitchClass::A,
            'b' => PitchClass::B,
            'c' => PitchClass::C,
            'd' => PitchClass::D,
            'e' => PitchClass::E,
            'f' => PitchClass::F,
            'g' => PitchClass::G,
            _ => return Err(()),
        });
    }
}

impl PitchClass {
    pub fn half_step_up(&self) -> Self {
        enum_iterator::next_cycle(self).unwrap()
    }

    pub fn half_step_down(&self) -> Self {
        enum_iterator::previous_cycle(self).unwrap()
    }

    pub fn half_steps_from_a(&self) -> usize {
        *self as usize - PitchClass::A as usize
    }
}

/// Length of note relative to base length
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Length {
    /// Base length
    Unit,
    Multiple(u64),
    Division(u64),
}
