use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig, StreamError};

pub fn play() -> Result<(), anyhow::Error> {
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
        SampleFormat::F32 => make_stream::<f32>(&device, &config),
        SampleFormat::I16 => make_stream::<i16>(&device, &config),
        SampleFormat::U16 => make_stream::<u16>(&device, &config),
    }?;

    // make sure playback has started (doesn't block, playback is in another thread)
    stream.play()?;

    // sleep for some time, since playback is in another thread
    std::thread::sleep(std::time::Duration::from_secs(30));

    println!("done sleeping");

    Ok(())

    // stream gets drop()'d
}

struct AudioGenerator {
    second_count: u32,
    sample_tick: f32,
    samples_per_second: f32,
    channels: usize,
}

impl AudioGenerator {
    fn tick_and_generate(&mut self) -> f32 {
        const MIDDLE_A_FREQUENCY: f32 = 440.0;
        let twelfth_root_of_two: f32 = f32::powf(2f32, 1f32/12f32);

        let half_steps_away = self.second_count % 12;

        // generate audio level from sine wave function
        let frequency = MIDDLE_A_FREQUENCY * f32::powi(twelfth_root_of_two, half_steps_away as i32);
        let x = self.sample_tick * frequency * std::f32::consts::TAU / self.samples_per_second;
        let mut amplitude = f32::sin(x);

        // reduce volume
        amplitude *= 1. / 20.;

        // increase tick (and roll over if needed)
        self.sample_tick = (self.sample_tick + 1.0) % self.samples_per_second;

        // if we rolled over
        if self.sample_tick == 0f32 {
            // increment second
            self.second_count += 1;
        }

        return amplitude;
    }

    fn new(samples_per_second: f32, channels: usize) -> Self {
        Self {
            second_count: 0,
            sample_tick: 0f32,
            samples_per_second,
            channels,
        }
    }

    fn fill_output<T: Sample>(&mut self, output: &mut [T], _cb_info: &cpal::OutputCallbackInfo) {
        for frame in output.chunks_mut(self.channels) {
            let next_value = self.tick_and_generate();
            for sample in frame.iter_mut() {
                *sample = Sample::from(&next_value);
            }
        }
    }
}

fn make_stream<T: Sample>(device: &Device, config: &cpal::StreamConfig) -> Result<Stream, anyhow::Error> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    println!(
        "run<{:?}>, rate: {}, channels: {}",
        T::FORMAT,
        sample_rate,
        channels
    );

    let mut val = AudioGenerator::new(sample_rate, channels);

    let data_callback = move |output: &mut [T], cb_info: &cpal::OutputCallbackInfo| {
        val.fill_output(output, cb_info);
    };

    let error_callback = |err: StreamError| {
        eprintln!("an error occurred on the output audio stream: {}", err);
    };

    let stream = device.build_output_stream(config, data_callback, error_callback)?;

    Ok(stream)
}
