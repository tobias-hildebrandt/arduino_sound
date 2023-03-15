use arduino_hal::simple_pwm::Prescaler;
use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PB5, PD0, PD1, PD5},
        Usart,
    },
    pac::USART0,
    port::{
        mode::{Input, Output},
        Pin,
    },
};

use core::mem::MaybeUninit;

/// struct representing all peripherals that we might want to access
pub struct Peripherals {
    pub serial: Usart<USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>,
    pub led: Pin<Output, PB5>,
    pub buzzer: Pin<Output, PD5>,
    pub clock: arduino_hal::hal::pac::TC1,
}

/// global variable, since we may need to access them inside the panic handler
static mut PERIPHS: MaybeUninit<Peripherals> = MaybeUninit::<_>::uninit();

/// initialize peripherals, *must* be called *once* before `get()`
pub unsafe fn init() {
    let peripherals = {
        let dp = arduino_hal::Peripherals::take().unwrap();

        let pins = arduino_hal::pins!(dp);

        let serial = arduino_hal::default_serial!(dp, pins, 57600);

        let led = pins.d13.into_output();

        let buzzer = pins.d5.into_output();

        let clock = dp.TC1;

        // enable interrupts
        unsafe { avr_device::interrupt::enable() };

        Peripherals {
            serial,
            led,
            buzzer,
            clock,
        }
    };

    // write it to the global variable
    PERIPHS.write(peripherals);
}

impl Peripherals {
    /// Set up the clock for buzzer output.
    /// Only needs to be run once, but it's OK to run it multiple times.
    pub fn setup_clock(&self) {
        // set up clock
        // CTC (clear timer on compare match) mode
        self.clock.tccr1a.write(|w| w.wgm1().bits(0b00));
        self.clock.tccr1b.write(|w| w.wgm1().bits(0b01));

        // enable interrupt on output control pin
        self.clock.timsk1.write(|w| w.ocie1a().set_bit());
    }

    /// Disable buzzer clock.
    pub fn disable_clock(&self) {
        // disable clock
        self.clock.tccr1b.write(|w| w.cs1().no_clock());

        // write output compare register
        self.clock.ocr1a.write(|w| w.bits(u16::MAX));

        // write zero to counter
        self.clock.tcnt1.write(|w| w.bits(0));

        // tear down global state
        avr_device::interrupt::free(|cs| {
            let state = AUDIO_STATE.borrow(cs);

            state.set(None);
        });
    }

    /// Set the frequency for the buzzer.
    /// Also starts the buzzer and begins the active/inactive cycle for the
    /// buzzer via interrupts.
    pub fn set_frequency(&mut self, frequency: f32) -> Result<(), ()> {
        // try to split frequency into active and inactive timer durations
        let durations =
            crate::timer_duration::ActiveInactiveTimerDurations::try_from_frequency(frequency)?;

        // print over serial
        // ufmt::uwriteln!(&mut self.serial, "durations: {}", &durations).unwrap();

        // set up global state
        avr_device::interrupt::free(|cs| {
            let state = AUDIO_STATE.borrow(cs);

            // set active time duration
            self.set_timer_durations(&durations.active);

            // raise pin high
            self.buzzer.set_high();

            // set state
            state.set(Some(AudioState {
                active: true,
                timer_durations: durations,
            }));
        });

        Ok(())
    }

    /// Write registers with timer duration data.
    fn set_timer_durations(&mut self, duration: &crate::timer_duration::TimerDuration) {
        // write zero to counter
        self.clock.tcnt1.write(|w| w.bits(0));

        // set up output compare register
        self.clock.ocr1a.write(|w| w.bits(duration.ticks()));

        // set running and prescaler
        self.clock.tccr1b.write(|w| match &duration.prescale() {
            Prescaler::Direct => w.cs1().direct(),
            Prescaler::Prescale8 => w.cs1().prescale_8(),
            Prescaler::Prescale64 => w.cs1().prescale_64(),
            Prescaler::Prescale256 => w.cs1().prescale_256(),
            Prescaler::Prescale1024 => w.cs1().prescale_1024(),
        });
    }
}

/// Global variable for audio state.
static AUDIO_STATE: avr_device::interrupt::Mutex<core::cell::Cell<Option<AudioState>>> =
    avr_device::interrupt::Mutex::new(core::cell::Cell::new(None));

/// Represents the current audio state and timer durations.
struct AudioState {
    /// are we active right now?
    active: bool,
    /// the durations of the active and inactive states
    timer_durations: crate::timer_duration::ActiveInactiveTimerDurations,
}

/// Interrupt service routine for timer1 comparison.
/// Switches between active and inactive buzzer state.
#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    avr_device::interrupt::free(|cs| {
        let periphs = unsafe { get() };
        let state_cell = AUDIO_STATE.borrow(cs);
        if let Some(mut state) = state_cell.take() {
            match state.active {
                true => {
                    // we are active, we must go inactive
                    state.active = false;

                    // set up timers for inactive
                    periphs.set_timer_durations(&state.timer_durations.inactive);

                    // set pin low
                    periphs.buzzer.set_low();
                }
                false => {
                    // we are inactive, we must go active
                    state.active = true;

                    // set up timers for active
                    periphs.set_timer_durations(&state.timer_durations.inactive);

                    // set pin high
                    periphs.buzzer.set_high();
                }
            }

            // update the state
            // TODO: split state so that we only have to write 1 boolean here?
            state_cell.set(Some(state));
        }
    });
}

/// returns reference to the peripherals, `init()` *must* be called *once* beforehand.
/// `unsafe` because we access a global.
pub unsafe fn get() -> &'static mut Peripherals {
    &mut *PERIPHS.as_mut_ptr()
}
