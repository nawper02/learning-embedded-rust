use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::hal::port::{Dynamic};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;
use crate::hardware::peripheral_abstraction::timer::{Timer, Prescaler::Prescale64};


const TRIGGER_UP_TIME: u16 = 10u16;

pub struct SonarSensor<T: Timer> {
    trig: Pin<Output, Dynamic>,
    echo: Pin<Input<Floating>, Dynamic>,
    timer: T,
}

impl<T: Timer> SonarSensor<T> {
    pub fn new(trig: Pin<Output, Dynamic>, echo: Pin<Input<Floating>, Dynamic>, timer: T) -> Self {
        Self { trig, echo, timer }
    }

    pub fn return_distance(&mut self) -> u16 {
        let mut delay = arduino_hal::Delay::new();

        // Start/prescale the timer
        self.timer.prescale(Prescale64);

        // Send out a pulse
        self.trig.set_high();
        delay.delay_us(TRIGGER_UP_TIME);
        self.trig.set_low();

        // Wait for echo to go high
        while self.echo.is_low() {
            if self.timer.read() >= 65000 { // Adjust this value based on your timer's resolution and timeout requirement
                // Stop the timer if it takes too long, assume no response (target too far)
                self.timer.stop();
                return 63500; // Return a specific value indicating out of range
            }
        }

        // Reset timer when echo goes high
        self.timer.reset();

        // Wait while echo is high and the timer counts
        while self.echo.is_high() {}

        // Stop the timer
        self.timer.stop();

        // Calculate and return the distance
        // The formula for distance will depend on the speed of sound and the timer's resolution
        // Here's a generic formula: (timer count) / constant factor
        self.timer.read() / 58 // Adjust the denominator based on your timing resolution and speed of sound
    }
}