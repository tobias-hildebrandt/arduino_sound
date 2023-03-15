#![no_std]
#![no_main]
// enable interrupts
#![feature(abi_avr_interrupt)]
// allow inline asm
#![feature(asm_experimental_arch)]

use ard_r_sound_base::Note;

/// module containing logic to set up peripherals.
mod peripherals;
/// module containing wrappers to serialize data.
mod print_wrappers;
/// module containing timer duration logic.
mod timer_duration;

ard_r_sound_macros::static_from_file! {OPTIMIZED, ../misc/example_abcs/mary.abc}

#[arduino_hal::entry]
fn main() -> ! {
    unsafe { peripherals::init() };

    let periphs = unsafe { peripherals::get() };

    // print_song(periphs);

    const BPM: f32 = 60.;
    const SINGLE_BEAT_SECS: f32 = BPM / 60.;

    periphs.setup_clock();

    ufmt::uwriteln!(&mut periphs.serial, "clock setup complete").unwrap();

    loop {
        // loop over all notes
        for (_i, unique_index) in OPTIMIZED.list.iter().enumerate() {
            let note: &Note = OPTIMIZED.uniques.get(*unique_index).unwrap();
            let freq = frequency(note);

            // the current note should be played for this many seconds
            let note_seconds_total = match note.length {
                ard_r_sound_base::Length::Unit => SINGLE_BEAT_SECS,
                ard_r_sound_base::Length::Multiple(m) => SINGLE_BEAT_SECS * m as f32,
                ard_r_sound_base::Length::Division(d) => SINGLE_BEAT_SECS / d as f32,
            };

            // ufmt::uwriteln!(
            //     &mut periphs.serial,
            //     "note #{}: uniq={}, freq={}, secs={}",
            //     _i,
            //     unique_index,
            //     print_wrappers::F32Wrapper(freq.unwrap_or(0f32)),
            //     print_wrappers::F32Wrapper(note_seconds_total)
            // )
            // .unwrap();

            match freq {
                Some(freq) => {
                    periphs.set_frequency(freq).unwrap();
                    arduino_hal::delay_us((note_seconds_total * 1_000_000.) as u32);
                    periphs.disable_clock();
                }
                None => {
                    periphs.disable_clock();
                    arduino_hal::delay_us((note_seconds_total * 1_000_000.) as u32);
                }
            }
        }
        periphs.disable_clock();
    }
}

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    // get peripherals
    let periphs = unsafe { peripherals::get() };

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

    // set LED high
    periphs.led.set_high();

    // loop forever
    loop {}
}

// calculate frequency of a note
pub fn frequency(note: &Note) -> Option<f32> {
    const MIDDLE_A_FREQUENCY: f32 = 440.0;
    // can't do a const powi here
    const TWELFTH_ROOT_OF_TWO: f32 = 1.059_463_1; // 12th root of 2 or 2^(1/12)

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

fn print_song(periphs: &mut peripherals::Peripherals) {
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
}
