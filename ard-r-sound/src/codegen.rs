use std::io::Write;
use std::{collections::HashMap, path::Path};

use tracing::info;

use crate::abc::{Length, Note, PitchOrRest, ABC};

#[macro_export]
macro_rules! HEADER_TEMPLATE {
    () => {
        r#"#ifndef ARDSOUND_SONG
#define ARDSOUND_SONG

// This is a song file

#include <note.h>

struct Note note_lookup[] = {{{}
}};

short song[] = {{
{}
}};

#endif
"#
    };
}

#[macro_export]
macro_rules! LOOKUP_LINE {
    () => {
        "\n\t{{ .pitch = {:>4}, .length = {:>2} }}{}"
    };
}

#[derive(Debug, Default)]
pub struct Optimized<'a> {
    pub uniques: Vec<&'a Note>,
    pub list: Vec<usize>,
}

impl<'a> From<&'a ABC> for Optimized<'a> {
    fn from(abc: &'a ABC) -> Self {
        let mut optimized = Self::default();

        // lookup table for list index
        let mut lookup: HashMap<&'a Note, usize> = HashMap::new();

        for note in abc.notes.iter() {
            if !optimized.uniques.contains(&note) {
                optimized.uniques.push(note);

                lookup.insert(note, optimized.uniques.len() - 1);
            }

            optimized.list.push(*lookup.get(note).unwrap());
        }

        optimized
    }
}

pub fn generate_c_header(abc: &ABC, file: &Path) -> Result<(), anyhow::Error> {
    let mut output_file = std::fs::File::create(file)?;

    let optimized = Optimized::from(abc);

    info!("{:#?}", optimized);

    let mut lookup_str = String::new();

    // add ending note
    lookup_str.push_str(&format!(LOOKUP_LINE!(), -127, 0, ","));

    let mut peekable = optimized.uniques.iter().peekable();
    while let Some(unique) = peekable.next() {
        lookup_str.push_str(&format!(
            LOOKUP_LINE!(),
            pitch_to_number(&unique.pitch),
            length_to_num(&unique.length),
            if let Some(_) = peekable.peek() {
                ","
            } else {
                ""
            }
        ));
    }

    let mut index_song_string = String::new();

    for index in optimized.list {
        index_song_string.push_str(&format!("\t{},\n", index));
    }

    // add ending note
    index_song_string.push_str("\t0");

    write!(
        output_file,
        HEADER_TEMPLATE!(),
        lookup_str, index_song_string
    )?;

    Ok(())
}

const NO_NOTE: i8 = 127;
fn pitch_to_number(pitch: &PitchOrRest) -> i16 {
    let pitch = match pitch {
        crate::abc::PitchOrRest::Pitch { class, octave } => {
            class.half_steps_from_a() as i8 + octave * 12
        }
        crate::abc::PitchOrRest::Rest => NO_NOTE,
    };
    pitch as i16
}

fn length_to_num(length: &Length) -> u16 {
    match length {
        Length::Division(32) => 0,
        Length::Division(16) => 1,
        Length::Division(8) => 2,
        Length::Division(4) => 3,
        Length::Division(2) => 4,
        Length::Unit => 5,
        Length::Multiple(2) => 6,
        Length::Multiple(4) => 7,
        Length::Multiple(8) => 8,
        Length::Multiple(16) => 9,
        _ => panic!("invalid length!"),
    }
}
