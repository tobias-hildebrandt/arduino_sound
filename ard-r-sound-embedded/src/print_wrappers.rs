pub struct NoteWrapper<'a>(pub &'a ard_r_sound_base::Note);
pub struct PitchOrRestWrapper<'a>(pub &'a ard_r_sound_base::PitchOrRest);
pub struct PitchClassWrapper<'a>(pub &'a ard_r_sound_base::PitchClass);
pub struct LengthWrapper<'a>(pub &'a ard_r_sound_base::Length);

impl ufmt::uDisplay for NoteWrapper<'_> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(f, "Note {{ pitch: ")?;
        ufmt::uwrite!(f, "{}", PitchOrRestWrapper(&self.0.pitch))?;
        ufmt::uwrite!(f, ", length: ")?;
        ufmt::uwrite!(f, "{}", LengthWrapper(&self.0.length))?;
        ufmt::uwrite!(f, " }}")?;

        Ok(())
    }
}

impl ufmt::uDisplay for PitchOrRestWrapper<'_> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self.0 {
            ard_r_sound_base::PitchOrRest::Pitch { class, octave } => {
                ufmt::uwrite!(
                    f,
                    "PitchOrRest::Pitch {{ class: {}, octave: {} }}",
                    PitchClassWrapper(class),
                    octave
                )
            }
            ard_r_sound_base::PitchOrRest::Rest => ufmt::uwrite!(f, "PitchOrRest::Rest"),
        }?;

        Ok(())
    }
}

impl ufmt::uDisplay for PitchClassWrapper<'_> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(
            f,
            "PitchClass::{}",
            match self.0 {
                ard_r_sound_base::PitchClass::A => "A",
                ard_r_sound_base::PitchClass::ASharpBFlat => "A#/Bb",
                ard_r_sound_base::PitchClass::B => "B",
                ard_r_sound_base::PitchClass::C => "C",
                ard_r_sound_base::PitchClass::CSharpDFlat => "C#/Db",
                ard_r_sound_base::PitchClass::D => "D",
                ard_r_sound_base::PitchClass::DSharpEFlat => "D#/Eb",
                ard_r_sound_base::PitchClass::E => "E",
                ard_r_sound_base::PitchClass::F => "F",
                ard_r_sound_base::PitchClass::FSharpGFlat => "F#/Gb",
                ard_r_sound_base::PitchClass::G => "G",
                ard_r_sound_base::PitchClass::GSharpAFlat => "G#/Ab",
            }
        )?;

        Ok(())
    }
}

impl ufmt::uDisplay for LengthWrapper<'_> {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        match self.0 {
            ard_r_sound_base::Length::Unit => ufmt::uwrite!(f, "Length::Unit"),
            ard_r_sound_base::Length::Multiple(x) => ufmt::uwrite!(f, "Length::Multiple: {}", x),
            ard_r_sound_base::Length::Division(x) => ufmt::uwrite!(f, "Length::Division: {}", x),
        }?;

        Ok(())
    }
}

pub struct F32Wrapper(pub f32);

impl ufmt::uDisplay for F32Wrapper {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let base = self.0 as u32;
        let decimal = self.0 - (self.0 as u32) as f32;
        ufmt::uwrite!(f, "~{}.{}", base, (decimal * 1000f32) as u32)
    }
}
