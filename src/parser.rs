use std::path::Path;

use anyhow::anyhow;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{abc, parse_tree};

#[derive(Parser)]
#[grammar = "abc.pest"]
pub struct ABCParser;

pub fn parse_abc(file_path: &Path) -> Result<abc::ABC, anyhow::Error> {
    let raw_file = std::fs::read_to_string(file_path)?;

    let entire = ABCParser::parse(Rule::Entire, &raw_file)?
        .next()
        .ok_or(anyhow!("parse iterator is empty?"))?;

    let mut version: Option<abc::Version> = None;
    let mut headers = abc::Headers::new();
    let mut notes: Vec<abc::Note> = Vec::new();

    for in_entire in entire.into_inner() {
        match in_entire.as_rule() {
            Rule::EOI => {
                println!("end of parse");
            }
            Rule::Version => {
                version = match in_entire.into_inner().next() {
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

                            //make sure we have an entry in our hashtable for the char
                            if !headers.contains_key(&key) {
                                headers.insert(key, Vec::new());
                            }
                            headers.get_mut(&key).unwrap().push(val.to_string());
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
                    let parse: parse_tree::NoteParse = parse_note(rules)?;
                    println!("parsed note is: {:?}", parse);
                    let note: abc::Note = parse.try_into()?;
                    println!("real note is: {:?}", note);
                    println!();
                    notes.push(note);
                }
                println!("done with body\n");
            }
            _ => unreachable!("matched a case in entire"),
        }
    }

    Ok(abc::ABC {
        version,
        headers,
        notes,
    })
}

fn parse_version(version: &str) -> Result<abc::Version, anyhow::Error> {
    let mut split = version.split(".");
    let first = split.next();
    let second = split.next();

    Ok(abc::Version {
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

fn parse_note(note: Pair<Rule>) -> Result<parse_tree::NoteParse, anyhow::Error> {
    println!("start note parse");

    let note_components: Vec<_> = note.into_inner().collect();

    let pitch: parse_tree::Pitch = note_components
        .iter()
        .filter(|c| matches!(c.as_rule(), Rule::NotePitch))
        .map(|r| parse_note_pitch(r.clone().into_inner().next().unwrap()))
        .next()
        .unwrap()?;

    let length: parse_tree::Length = note_components
        .iter()
        .filter(|c| matches!(c.as_rule(), Rule::NoteLength))
        .map(|r| parse_note_length(r.as_str()))
        .next()
        .unwrap()?;

    let note = parse_tree::NoteParse { pitch, length };

    println!("end note parse");

    Ok(note)
}

fn parse_note_pitch(note_pitch: Pair<Rule>) -> Result<parse_tree::Pitch, anyhow::Error> {
    // handle rest first
    if matches!(note_pitch.as_rule(), Rule::RestChar) {
        return Ok(parse_tree::Pitch::Rest);
    }

    let pitch_components: Vec<Pair<Rule>> = note_pitch.into_inner().collect();

    let pitch_char: char = pitch_components
        .iter()
        .filter(|c| matches!(c.as_rule(), Rule::PitchChar))
        .map(|r| r.as_str().chars().next().unwrap())
        .next()
        .unwrap();

    let accidentals = pitch_components
        .iter()
        .filter(|c| matches!(c.as_rule(), Rule::Accidental))
        .map(|r| match r.as_str().chars().next().unwrap() {
            '^' => parse_tree::Accidental::Sharp,
            '_' => parse_tree::Accidental::Flat,
            '=' => parse_tree::Accidental::Natural,
            _ => unreachable!(),
        })
        .collect();

    let octaves = pitch_components
        .iter()
        .filter(|c| matches!(c.as_rule(), Rule::Octave))
        .map(|r| match r.as_str().chars().next().unwrap() {
            ',' => parse_tree::Octave::Down,
            '\'' => parse_tree::Octave::Up,
            _ => unreachable!(),
        })
        .collect();

    Ok(parse_tree::Pitch::NonRest {
        accidentals,
        pitch_char,
        octaves,
    })
}

fn parse_note_length(note_length: &str) -> Result<parse_tree::Length, anyhow::Error> {
    // easier just to parse as string
    if note_length.len() == 0 {
        return Ok(parse_tree::Length::Default);
    }

    let slashes_count = note_length.chars().take_while(|c| *c == '/').count();
    let numbers: String = note_length
        .trim()
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    Ok(match slashes_count {
        0 => {
            // positive multiplier
            parse_tree::Length::Specified {
                divided: false,
                number: match numbers.len() {
                    0 => 1,
                    _ => numbers.parse()?,
                },
            }
        }
        1 => parse_tree::Length::Specified {
            divided: true,
            number: match numbers.len() {
                0 => 1,
                _ => numbers.parse()?,
            },
        },
        _ => parse_tree::Length::Specified {
            divided: true,
            number: 2u64.pow(slashes_count.try_into().unwrap()),
        },
    })
}
