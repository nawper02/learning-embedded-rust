use avr_device::atmega2560::{TC1};

pub trait Timer {

    /// Prescales the timer.
    fn prescale(&self, prescaler: Prescaler);

    /// Resets the timer to zero.
    fn reset(&self);

    /// Reads the current timer value.
    fn read(&self) -> u16;

    /// Stops the timer.
    fn stop(&self);
}

// Implementing Timer trait for each timer will be a lot of boilerplate since we must call self.tccr1b.....
// But it is a necessary evil to clean up our code elsewhere.
impl Timer for TC1 {
    fn prescale(&self, prescaler: Prescaler) {
        match prescaler {
            Prescaler::Prescale8 => self.tccr1b.write(|w| w.cs1().prescale_8()),
            Prescaler::Prescale64 => self.tccr1b.write(|w| w.cs1().prescale_64()),
            Prescaler::Prescale256 => self.tccr1b.write(|w| w.cs1().prescale_256()),
            Prescaler::Prescale1024 => self.tccr1b.write(|w| w.cs1().prescale_1024()),
        }
    }

    fn reset(&self) {
        self.tcnt1.write(|w| w.bits(0));
    }

    fn read(&self) -> u16 {
        self.tcnt1.read().bits()
    }

    fn stop(&self) {
        // Code to stop TC1 timer
        self.tccr1b.write(|w| w.cs1().no_clock());
    }
}

pub enum Prescaler {
    Prescale8,
    Prescale64,
    Prescale256,
    Prescale1024,
}
