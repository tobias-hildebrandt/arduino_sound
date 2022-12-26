use anyhow::anyhow;

use pest::Parser;
use pest_derive::Parser;

use crate::abc::{Headers, Length, Note, Version, ABC};

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
                for note in body {
                    println!("start note");
                    for note_component in note.into_inner() {
                        match note_component.as_rule() {
                            Rule::note_pitch => {
                                println!("start note pitch");
                                let pitch_components = note_component.into_inner();
                                for pitch_component in pitch_components {
                                    match pitch_component.as_rule() {
                                        Rule::accidental => {
                                            println!("accidental: {:?}", pitch_component.as_str());
                                        }
                                        Rule::pitch_char => {
                                            println!("pitch: {:?}", pitch_component.as_str());
                                        }
                                        Rule::rest_char => {
                                            println!("rest");
                                        }
                                        Rule::octave => {
                                            println!("octave: {:?}", pitch_component.as_str());
                                        }
                                        _ => unreachable!(),
                                    }
                                }
                                println!("end note pitch");
                            }
                            Rule::note_length => {
                                println!("TODO note length: {:?}", note_component.into_inner().as_str());
                            }
                            _ => unreachable!(),
                        }
                    }
                    println!("end note\n");
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

// fn parse_note(pitch: &str, length: &str) -> Result(Note, anyhow::Error) {
//     Ok(Note {
//         pitch: pitch.parse()?,
//         length: Length::One,
//     })
// }
