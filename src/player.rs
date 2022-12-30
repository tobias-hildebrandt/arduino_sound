use std::io::Write;

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

    /*
    (from cpal docs)
    Note: Creating and running a stream will not block the thread.
    On modern platforms, the given callback is called by a dedicated,
    high-priority thread responsible for delivering audio data to the
    system’s audio device in a timely manner. On older platforms that
    only provide a blocking API (e.g. ALSA), CPAL will create a thread
    in order to consistently provide non-blocking behaviour
    (currently this is a thread per stream, but this may change to use a
    single thread for all streams).
    */
    stream.play()?;

    // sleep for some time, since playback is in another thread
    std::thread::sleep(std::time::Duration::from_secs(60));

    println!("done sleeping");

    Ok(())

    // stream gets drop()'d
}

struct AudioGenerator {
    abc: ABC,
    note_index: Option<usize>,
    samples_played_in_note: u32,
    samples_played_this_second: u32,
    samples_per_second: u32,
    channels: usize,
}

impl AudioGenerator {
    fn generate_and_tick(&mut self) -> Option<f32> {
        const MIDDLE_A_FREQUENCY: f64 = 440.0;
        const BPM: f64 = 60.;
        let twelfth_root_of_two: f64 = f64::powf(2., 1. / 12.);

        let mut new_note = false;

        let mut current_note_seconds;

        // advance note_index until we get to a note that needs to be played
        let current_note = loop {
            let index = match self.note_index {
                Some(i) => i,
                None => {
                    new_note = true;
                    self.note_index = Some(0);
                    0
                }
            };
            let current_note = self.abc.notes.get(index)?;

            let single_beat_secs = BPM / 60. / 4.;

            // the current note should be played for this many seconds
            current_note_seconds = match current_note.length {
                crate::abc::Length::Unit => single_beat_secs,
                crate::abc::Length::Multiple(m) => single_beat_secs * m as f64,
                crate::abc::Length::Division(d) => single_beat_secs / d as f64,
            };

            // the current note should be played for this many samples
            let current_note_total_samples =
                (current_note_seconds * self.samples_per_second as f64) as u32;

            if self.samples_played_in_note >= current_note_total_samples {
                // go to the next note
                self.samples_played_in_note = 0;
                self.note_index = Some(index + 1);
                new_note = true;
            } else {
                // this is the note we want
                break self.abc.notes.get(index)?;
            }
        };

        // output for this sample
        let mut amplitude;

        match &current_note.pitch {
            crate::abc::PitchOrRest::Pitch { class, octave } => {
                let half_steps_away = (*octave as i32 * 12) + class.half_steps_from_a() as i32;

                // generate audio level from sine wave function

                // desired frequency of the note
                let frequency =
                    MIDDLE_A_FREQUENCY * f64::powi(twelfth_root_of_two, half_steps_away);

                if new_note {
                    println!(
                        "note at index {:0>2?} = {:?}, half steps = {}, freq = {}, seconds = {}",
                        self.note_index.unwrap(),
                        current_note,
                        half_steps_away,
                        frequency,
                        current_note_seconds
                    );
                }

                // x coord on the sine wave
                let x = self.samples_played_this_second as f64 * frequency * std::f64::consts::TAU
                    / self.samples_per_second as f64;

                // y coord on the sine wave
                amplitude = f64::sin(x);

                // reduce volume
                amplitude *= 1. / 20.;
            }
            crate::abc::PitchOrRest::Rest => {
                if new_note {
                    println!("rest at index: {:?}", self.note_index);
                }

                // silence has an amplitude of 0?
                amplitude = 0.;
            }
        };

        // increase tick (and roll over if needed)
        self.samples_played_this_second =
            (self.samples_played_this_second + 1) % self.samples_per_second;

        self.samples_played_in_note += 1;

        return Some(amplitude as f32);
    }

    fn new(abc: ABC, samples_per_second: u32, channels: usize) -> Self {
        Self {
            abc,
            note_index: None,
            samples_played_in_note: 0,
            samples_played_this_second: 0,
            samples_per_second,
            channels,
        }
    }

    fn fill_output<T: Sample>(&mut self, output: &mut [T]) {
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

    let mut audio_generator = AudioGenerator::new(abc, sample_rate, channels);

    let data_callback = move |output: &mut [T], cb_info: &cpal::OutputCallbackInfo| {
        audio_generator.fill_output(output);
    };

    let error_callback = |err: StreamError| {
        eprintln!("an error occurred on the output audio stream: {}", err);
    };

    let stream = device.build_output_stream(config, data_callback, error_callback)?;

    Ok(stream)
}

// write output of i16, single channel audio @ 44100 Hz to file
pub fn write_output_to_file(abc: ABC, filename: &str) -> Result<(), anyhow::Error> {
    const BUFFER_SIZE: usize = 65535;
    let mut buffer = [0i16; BUFFER_SIZE];
    let mut byte_buffer = [0u8; BUFFER_SIZE * 4];

    let sample_rate = 44100;
    let channels = 1usize;

    let total_samples = sample_rate * 30;
    let mut current_samples = 0;

    let mut audio_generator = AudioGenerator::new(abc, sample_rate, channels);

    use std::fs::File;
    let mut file = File::create(filename)?;

    while current_samples < total_samples {
        audio_generator.fill_output(&mut buffer);

        // extract bytes
        for (index, num) in buffer.iter().enumerate() {
            let bytes = num.to_le_bytes();
            byte_buffer[index * 2] = bytes[0];
            byte_buffer[index * 2 + 1] = bytes[1];
        }

        // write entire byte buffer
        file.write(&byte_buffer)?;

        current_samples += BUFFER_SIZE as u32;
    }

    Ok(())
}
