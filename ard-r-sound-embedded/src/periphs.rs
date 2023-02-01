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
}

/// global variable, since we may need to access them inside the panic handler
static mut PERIPHS: MaybeUninit<Peripherals> = MaybeUninit::<_>::uninit();

/// initialize peripherals, *must* be called *once* before `get_singletons()`
pub unsafe fn init() {
    let peripherals = {
        let dp = arduino_hal::Peripherals::take().unwrap();

        let pins = arduino_hal::pins!(dp);

        let serial = arduino_hal::default_serial!(dp, pins, 57600);

        let led = pins.d13.into_output();

        let buzzer = pins.d5.into_output();

        Peripherals {
            serial,
            led,
            buzzer,
        }
    };

    // write it to the global variable
    PERIPHS.write(peripherals);
}

/// returns reference to the peripherals, `init()` *must* be called *once* beforehand.
/// `unsafe` because we access a global
pub unsafe fn get() -> &'static mut Peripherals {
    unsafe { &mut *PERIPHS.as_mut_ptr() }
}
