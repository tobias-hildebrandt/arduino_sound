use std::str::FromStr;

use anyhow::anyhow;

use fraction::Fraction;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::abc::{Headers, Length, Note, Pitch, PitchClass, PitchOrRest, Version, ABC};

#[derive(Parser)]
#[grammar = "abc.pest"]
pub struct ABCParser;

pub fn parse_abc(file_path: &str) -> Result<ABC, anyhow::Error> {
    let raw_file = std::fs::read_to_string(file_path)?;

    let entire = ABCParser::parse(Rule::Entire, &raw_file)?
        .next()
        .ok_or(anyhow!("parse iterator is empty?"))?;

    let mut abc = ABC::default();

    for in_entire in entire.into_inner() {
        match in_entire.as_rule() {
            Rule::EOI => {
                println!("end of parse");
            }
            Rule::Version => {
                abc.version = match in_entire.into_inner().next() {
                    Some(pair) => match parse_version(pair.as_str()) {
                        Ok(v) => {
                            println!("parsed version as: {:?}", v);
                            Some(v)
                        }
                        Err(e) => {
                            println!("unable to parse version: {}", e);
                            None
                        }
                    },
                    None => {
                        println!("no version");
                        None
                    }
                };
            }
            Rule::Information => {
                println!("information:");
                for info in in_entire.into_inner() {
                    let mut inner = info.into_inner();
                    match (inner.next(), inner.next()) {
                        (Some(key), Some(val)) => {
                            let (key, val) = parse_information(key.as_str(), val.as_str())?;
                            println!("parsed info line as: key: {:?}, val: {:?}", key, val);

                            // make sure we have an entry in our hashtable for the char
                            if !abc.headers.contains_key(&key) {
                                abc.headers.insert(key, Vec::new());
                            }
                            abc.headers.get_mut(&key).unwrap().push(val.to_string());
                        }
                        _ => println!("invalid information field: {:?}", inner.as_str()),
                    }
                }
                println!("done with information\n");
            }
            Rule::Body => {
                let body = in_entire.into_inner();
                println!("body:");
                for rules in body {
                    let note = parse_note(rules)?;
                }
                println!("done with body\n");
            }
            _ => unreachable!("matched a case in entire"),
        }
    }

    Ok(abc)
}

fn parse_version(version: &str) -> Result<Version, anyhow::Error> {
    let mut split = version.split(".");
    let first = split.next();
    let second = split.next();

    Ok(Version {
        major: first
            .ok_or(anyhow!(
                "Invalid version string, could not parse major version: \'{}\'",
                version
            ))?
            .parse()?,
        minor: second
            .ok_or(anyhow!(
                "Invalid version string, could not parse minor version: \'{}\'",
                version
            ))?
            .parse()?,
    })
}

fn parse_information<'a>(key: &'a str, val: &'a str) -> Result<(char, &'a str), anyhow::Error> {
    if key.chars().count() > 1 {
        return Err(anyhow!("key has more than 1 char"));
    }
    let key = key.chars().next().unwrap();

    Ok((key, val))
}

fn parse_note(rules: Pair<Rule>) -> Result<Note, anyhow::Error> {
    let mut pitch = PitchOrRest::Rest;
    let mut length = Fraction::new(1u8, 1u8);

    println!("start note");
    for note_component in rules.into_inner() {
        match note_component.as_rule() {
            Rule::NotePitch => {
                println!("start note pitch");
                let pitch_full = note_component.into_inner().next().unwrap();
                match pitch_full.as_rule() {
                    Rule::RestChar => {}
                    Rule::non_rest_note_pitch => {
                        let pitch_components = pitch_full.into_inner();
                        for pitch_component in pitch_components {
                            match pitch_component.as_rule() {
                                Rule::RestChar => {
                                    println!("rest");
                                    pitch = PitchOrRest::Rest;
                                }
                                Rule::PitchChar => {
                                    println!("pitch: {:?}", pitch_component.as_str());

                                    let p: PitchClass =
                                        PitchClass::from_str(pitch_component.as_str())
                                            .map_err(|_| anyhow!("failed pitch parsing"))?;
                                    pitch = PitchOrRest::Pitch(Pitch {
                                        class: p,
                                        octave: 0,
                                    });
                                }
                                Rule::Accidental => {
                                    println!("accidental: {:?}", pitch_component.as_str());
                                    if let PitchOrRest::Pitch(ref mut p) = &mut pitch {
                                        match pitch_component.as_str().chars().next().unwrap() {
                                            '^' => p.class = p.class.half_step_up(),
                                            '_' => p.class = p.class.half_step_down(),
                                            _ => unreachable!(),
                                        }
                                    }
                                }
                                Rule::Natural => {
                                    println!("natural");
                                }
                                Rule::Octave => {
                                    println!("octave: {:?}", pitch_component.as_str());
                                }
                                _ => unreachable!("pitch fail: {:?}", pitch_component.as_str()),
                            }
                        }
                    }
                    _ => unreachable!("pitch fail: {:?}", pitch_full.as_str()),
                }

                println!("end note pitch");
            }
            Rule::NoteLength => {
                println!(
                    "TODO note length: {:?}",
                    note_component.into_inner().as_str()
                );
            }
            _ => unreachable!(),
        }
    }

    let note = Note {
        pitch,
        length: Length::One,
    };

    println!("end note: {:?}\n", note);

    Ok(note)
}
