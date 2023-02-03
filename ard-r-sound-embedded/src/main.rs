#![no_std]
#![no_main]

use ard_r_sound_base::Note;

/// module containing logic to set up peripherals.
mod periphs;
/// module containing wrappers to serialize data.
mod print_wrappers;

ard_r_sound_macros::static_from_file!{OPTIMIZED, ../misc/example_abcs/mary.abc}

#[arduino_hal::entry]
fn main() -> ! {
    unsafe { periphs::init() };

    let periphs = unsafe { periphs::get() };

    // print uniques
    for (i, unique) in OPTIMIZED.uniques.iter().enumerate() {
        ufmt::uwriteln!(
            &mut periphs.serial,
            "unique #{}: {}",
            i,
            print_wrappers::NoteWrapper(unique)
        )
        .unwrap()
    }

    // print note list
    for (i, index) in OPTIMIZED.list.iter().enumerate() {
        ufmt::uwriteln!(&mut periphs.serial, "list #{}: {}", i, index).unwrap();
    }

    const BPM: f32 = 60.;
    const SINGLE_BEAT_SECS: f32 = BPM / 60.;

    // taken from std::f32::consts::TAU
    // pub const TAU: f32 = 6.28318530717958647692528676655900577f32; // 6.28318548f32
    loop {
        // loop over all notes
        for (i, unique_index) in OPTIMIZED.list.iter().enumerate() {
            let note: &Note = OPTIMIZED.uniques.get(*unique_index).unwrap();
            let freq = frequency(note);

            // the current note should be played for this many seconds
            let note_seconds_total = match note.length {
                ard_r_sound_base::Length::Unit => SINGLE_BEAT_SECS,
                ard_r_sound_base::Length::Multiple(m) => SINGLE_BEAT_SECS * m as f32,
                ard_r_sound_base::Length::Division(d) => SINGLE_BEAT_SECS / d as f32,
            };

            ufmt::uwriteln!(
                &mut periphs.serial,
                "note #{}: uniq={}, freq={}, secs={}",
                i,
                unique_index,
                print_wrappers::F32Wrapper(freq.unwrap_or(0f32)),
                print_wrappers::F32Wrapper(note_seconds_total)
            )
            .unwrap();

            match freq {
                Some(freq) => {
                    tone((note_seconds_total * 1_000_000.) as u32, freq, periphs);
                }
                None => {
                    arduino_hal::delay_us((note_seconds_total * 1_000_000f32) as u32);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    let periphs = unsafe { periphs::get() };

    // try to send panic info over serial
    match panic_info.location() {
        Some(location) => ufmt::uwriteln!(
            &mut periphs.serial,
            "panic @ {}:{}",
            location.file(),
            location.line()
        )
        .unwrap(),
        None => ufmt::uwriteln!(&mut periphs.serial, "panic, but unable to get location").unwrap(),
    }

    // blink 50 times
    for _ in 0..50 {
        periphs.led.toggle();
        arduino_hal::delay_ms(50);
    }

    // set low and loop forever
    periphs.led.set_low();
    loop {}
}

// calculate frequency of a note
pub fn frequency(note: &Note) -> Option<f32> {
    const MIDDLE_A_FREQUENCY: f32 = 440.0;
    // can't do a const powi here
    const TWELFTH_ROOT_OF_TWO: f32 = 1.0594630943592952645618252949463; // 12th root of 2 or 2^(1/12)

    match note.pitch {
        ard_r_sound_base::PitchOrRest::Pitch { class, octave } => {
            let half_steps_away = (octave as i32 * 12) + class.half_steps_from_a() as i32;

            let frequency =
                MIDDLE_A_FREQUENCY * micromath::F32Ext::powi(TWELFTH_ROOT_OF_TWO, half_steps_away);

            Some(frequency)
        }
        ard_r_sound_base::PitchOrRest::Rest => None,
    }
}

// TODO: implement tone() via clock
// à la https://github.com/arduino/ArduinoCore-avr/blob/master/cores/arduino/Tone.cpp
pub fn tone(length_us: u32, freq: f32, periphs: &mut periphs::Peripherals) {
    let mut remaining_us = length_us;

    // period of the wave
    let period = 1. / freq;

    // microseconds
    let period_us = (period * 1_000_000.) as u32;

    // for what fraction of the period will we set the pin high
    const ACTIVE_FRACTION: f32 = 0.02;

    let active_us: u32 = (period_us as f32 * ACTIVE_FRACTION) as u32;

    while remaining_us > 0 {
        periphs.buzzer.set_high();
        arduino_hal::delay_us(active_us);
        periphs.buzzer.set_low();
        arduino_hal::delay_us(period_us - active_us);

        if remaining_us < period_us {
            // can't subtract or else we will underflow
            break;
        } else {
            remaining_us -= period_us;
        }
    }
}
