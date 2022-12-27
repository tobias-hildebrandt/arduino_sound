#[derive(Debug)]
pub(crate) struct NoteParse {
    pub(crate) pitch: Pitch,
    pub(crate) length: Length,
}

impl TryInto<crate::abc::Note> for NoteParse {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<crate::abc::Note, Self::Error> {
        Ok(crate::abc::Note {
            pitch: self.pitch.try_into()?,
            length: self.length.try_into()?,
        })
    }
}

#[derive(Debug)]
pub(crate) enum Pitch {
    Rest,
    NonRest {
        accidentals: Vec<Accidental>,
        pitch_char: char,
        octaves: Vec<Octave>,
    },
}

impl TryInto<crate::abc::PitchOrRest> for Pitch {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<crate::abc::PitchOrRest, Self::Error> {
        Ok(match self {
            Pitch::Rest => crate::abc::PitchOrRest::Rest,
            Pitch::NonRest {
                accidentals,
                pitch_char,
                octaves,
            } => {
                let mut class: crate::abc::PitchClass = pitch_char.try_into().map_err(|e| {
                    anyhow::anyhow!("unable to convert char into pitchclass?: {:?}", e)
                })?;
                for accidental in accidentals {
                    class = match accidental {
                        Accidental::Natural => class,
                        Accidental::Flat => class.half_step_down(),
                        Accidental::Sharp => class.half_step_up(),
                    }
                }

                let octave = octaves
                    .iter()
                    .map(|o| match o {
                        Octave::Up => 1,
                        Octave::Down => -1,
                    })
                    .fold(0, |accum, n| accum + n);

                crate::abc::PitchOrRest::Pitch { class, octave }
            }
        })
    }
}

#[derive(Debug)]
pub(crate) enum Accidental {
    Natural,
    Flat,
    Sharp,
}

#[derive(Debug)]
pub(crate) enum Octave {
    Up,
    Down,
}

#[derive(Debug)]
pub(crate) enum Length {
    Default,
    Specified { divided: bool, number: u64 },
}

impl TryInto<crate::abc::Length> for Length {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<crate::abc::Length, Self::Error> {
        Ok(match self {
            Length::Default => crate::abc::Length::Unit,
            Length::Specified { divided, number } => match divided {
                true => crate::abc::Length::Multiple(number),
                false => crate::abc::Length::Division(number),
            },
        })
    }
}
