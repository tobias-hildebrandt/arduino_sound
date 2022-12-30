use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig, StreamError};

use crate::abc::ABC;

pub fn play(abc: ABC) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");
    let output_config = device
        .default_output_config()
        .expect("no default device output config");

    let sample_format = output_config.sample_format();
    let config: StreamConfig = output_config.into();

    println!("device: {:?}, config: {:?}", device.name(), config);

    let stream = match sample_format {
        SampleFormat::F32 => make_stream::<f32>(&device, &config, abc),
        SampleFormat::I16 => make_stream::<i16>(&device, &config, abc),
        SampleFormat::U16 => make_stream::<u16>(&device, &config, abc),
    }?;

    // make sure playback has started (doesn't block, playback is in another thread)
    stream.play()?;

    // sleep for some time, since playback is in another thread
    std::thread::sleep(std::time::Duration::from_secs(60));

    println!("done sleeping");

    Ok(())

    // stream gets drop()'d
}

struct AudioGenerator {
    abc: ABC,
    note_index: usize,
    samples_played_in_note: u32,
    samples_played_this_second: u32,
    samples_per_second: u32,
    channels: usize,
}

impl AudioGenerator {
    fn generate_and_tick(&mut self) -> Option<f32> {
        const MIDDLE_A_FREQUENCY: f32 = 440.0;
        const BPM: f32 = 60.;
        let twelfth_root_of_two: f32 = f32::powf(2f32, 1f32 / 12f32);

        // advance note_index until we get to a note that needs to be played
        let current_note = loop {
            let current_note = self.abc.notes.get(self.note_index)?;

            let single_beat_secs = BPM / 60. / 4.;

            // the current note should be played for this many seconds
            let current_note_seconds = match current_note.length {
                crate::abc::Length::Unit => single_beat_secs,
                crate::abc::Length::Multiple(m) => single_beat_secs * m as f32,
                crate::abc::Length::Division(d) => single_beat_secs / d as f32,
            };

            // the current note should be played for this many samples
            let current_note_total_samples =
                (current_note_seconds * self.samples_per_second as f32) as u32;

            if self.samples_played_in_note >= current_note_total_samples {
                // go to the next note
                self.samples_played_in_note = 0;
                self.note_index += 1;
                println!(
                    "(changing notes) note #{} should be played for {} seconds = {} samples",
                    self.note_index, current_note_seconds, current_note_total_samples
                );
            } else {
                // this is the note we want
                break self.abc.notes.get(self.note_index)?;
            }
        };

        // output for this sample
        let mut amplitude;

        match &current_note.pitch {
            crate::abc::PitchOrRest::Pitch { class, octave } => {
                let half_steps_away = (*octave as i32 * 12) + class.half_steps_from_a() as i32;

                // println!(
                //     "note at index {:?}, half steps = {}",
                //     self.note_index, half_steps_away
                // );

                // generate audio level from sine wave function

                // desired frequency of the note
                let frequency =
                    MIDDLE_A_FREQUENCY * f32::powi(twelfth_root_of_two, half_steps_away);
                // println!(
                //     "note at index {:?}, frequency = {}",
                //     self.note_index, frequency
                // );

                // x coord on the sine wave
                let x = self.samples_played_this_second as f32 * frequency * std::f32::consts::TAU
                    / self.samples_per_second as f32;

                // y coord on the sine wave
                amplitude = f32::sin(x);

                // reduce volume
                amplitude *= 1. / 20.;
            }
            crate::abc::PitchOrRest::Rest => {
                // println!("rest at index: {:?}", self.note_index);

                // silence has an amplitude of 0?
                amplitude = 0.;
            }
        };

        // increase tick (and roll over if needed)
        self.samples_played_this_second =
            (self.samples_played_this_second + 1) % self.samples_per_second;

        self.samples_played_in_note += 1;

        return Some(amplitude);
    }

    fn new(abc: ABC, samples_per_second: u32, channels: usize) -> Self {
        Self {
            abc,
            note_index: 0,
            samples_played_in_note: 0,
            samples_played_this_second: 0,
            samples_per_second,
            channels,
        }
    }

    fn fill_output<T: Sample>(&mut self, output: &mut [T], _cb_info: &cpal::OutputCallbackInfo) {
        // println!("num frames: {}", output.len());
        for frame in output.chunks_mut(self.channels) {
            let next_value = match self.generate_and_tick() {
                Some(v) => v,
                None => {
                    // TODO: oneshot signal to end stream
                    0f32
                }
            };
            // do the same thing for each channel
            for sample in frame.iter_mut() {
                *sample = Sample::from(&next_value);
            }
        }
    }
}

fn make_stream<T: Sample>(
    device: &Device,
    config: &cpal::StreamConfig,
    abc: ABC,
) -> Result<Stream, anyhow::Error> {
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;

    println!(
        "run<{:?}>, rate: {}, channels: {}",
        T::FORMAT,
        sample_rate,
        channels
    );

    let mut val = AudioGenerator::new(abc, sample_rate, channels);

    let data_callback = move |output: &mut [T], cb_info: &cpal::OutputCallbackInfo| {
        val.fill_output(output, cb_info);
    };

    let error_callback = |err: StreamError| {
        eprintln!("an error occurred on the output audio stream: {}", err);
    };

    let stream = device.build_output_stream(config, data_callback, error_callback)?;

    Ok(stream)
}
