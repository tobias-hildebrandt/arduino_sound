use std::{collections::HashMap, str::FromStr};

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

/// The version of an ABC file
#[derive(Debug, Clone)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

pub type Headers = HashMap<char, Vec<String>>;

/// A single note
/// TODO: maybe call it a "tone" instead?
#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: PitchOrRest,
    pub length: Length,
}

/// Musical pitch or a rest
#[derive(Debug, Clone)]
pub enum PitchOrRest {
    Pitch(Pitch),
    Rest,
}

/// Pitch of a note
#[derive(Debug, Clone)]
pub struct Pitch {
    pub class: PitchClass,
    /// Octaves above or below the base octave
    pub octave: i8,
}

/// Twelve-tone pitch class
/// TODO: is this relative to key?
#[derive(Debug, Clone)]
pub enum PitchClass {
    C,
    CSharpDFlat,
    D,
    DSharpEFlat,
    E,
    F,
    FSharpGFlat,
    G,
    GSharpAFlat,
    A,
    ASharpBFlat,
    B,
}

/// Length of note relative to base length
#[derive(Debug, Clone)]
pub enum Length {
    /// Base length
    One,
    Multiple(u64),
    Division(u64),
}
