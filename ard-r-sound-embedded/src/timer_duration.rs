use arduino_hal::{simple_pwm::Prescaler, clock::Clock};

// arduino clock rate
const CLOCK_RATE: f32 = arduino_hal::DefaultClock::FREQ as f32;

/// Represents a timer duration.
pub struct TimerDuration {
    /// Private field, use ticks()
    ticks: u16,
    /// Private field, use prescale()
    prescale: Prescaler,
}

impl TimerDuration {
    pub fn ticks(&self) -> u16 {
        self.ticks
    }

    pub fn prescale(&self) -> Prescaler {
        self.prescale
    }

    pub fn total_ticks(&self) -> u32 {
        self.ticks as u32
            * match self.prescale {
                Prescaler::Direct => 1,
                Prescaler::Prescale8 => 8,
                Prescaler::Prescale64 => 64,
                Prescaler::Prescale256 => 256,
                Prescaler::Prescale1024 => 1024,
            }
    }

    pub fn seconds(&self) -> f32 {
        self.total_ticks() as f32 / CLOCK_RATE
    }
}

impl ufmt::uDisplay for TimerDuration {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(
            f,
            "TimerDuration {{ ticks: {}, prescale: {}, seconds() = {} }}",
            self.ticks,
            match self.prescale {
                Prescaler::Direct => 1,
                Prescaler::Prescale8 => 8,
                Prescaler::Prescale64 => 64,
                Prescaler::Prescale256 => 256,
                Prescaler::Prescale1024 => 1024,
            },
            crate::print_wrappers::F32Wrapper(self.seconds())
        )
    }
}

impl TimerDuration {
    fn try_from_ticks(ticks: u32) -> Result<Self, ()> {
        let mut counter = ticks;
        let mut prescale: u32 = 1;

        while counter > (u16::MAX as u32) {
            counter /= 8;
            prescale *= 8;
        }

        let prescale = match prescale {
            1 => Prescaler::Direct,
            8 => Prescaler::Prescale8,
            64 => Prescaler::Prescale64,
            256 => Prescaler::Prescale256,
            1024 => Prescaler::Prescale1024,
            _ => return Err(()),
        };

        Ok(TimerDuration {
            ticks: counter as u16,
            prescale,
        })
    }
}

/// Two timer durations, one for active and one for inactive.
pub struct ActiveInactiveTimerDurations {
    pub active: TimerDuration,
    pub inactive: TimerDuration,
}

impl ActiveInactiveTimerDurations {
    pub fn try_from_frequency(frequency: f32) -> Result<Self, ()> {
        // fraction of the period that the pin should be high
        const ACTIVE_FRACTION: f32 = 0.01;

        let ticks_per_period = (CLOCK_RATE / frequency) as u32;
        let active_ticks = (ticks_per_period as f32 * ACTIVE_FRACTION) as u32;
        let inactive_ticks = ticks_per_period - active_ticks;

        Ok(Self {
            active: TimerDuration::try_from_ticks(active_ticks)?,
            inactive: TimerDuration::try_from_ticks(inactive_ticks)?,
        })
    }
}

impl ufmt::uDisplay for ActiveInactiveTimerDurations {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        ufmt::uwrite!(
            f,
            "ActiveInactiveTimerDurations {{ active: {}, inactive: {} }}",
            self.active,
            self.inactive,
        )
    }
}
