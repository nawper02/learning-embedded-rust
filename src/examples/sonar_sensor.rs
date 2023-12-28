/*
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod hardware;
mod tools;

use hardware::sensors::sonar::SonarSensor;

use arduino_hal::prelude::*;
use embedded_hal::blocking::delay::DelayUs;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Initialize TC1 for the SonarSensor
    let timer = dp.TC1;

    // Prescale TC1
    timer.tccr1b.write(|w| w.cs1().prescale_64());

    // Define Trigger and Echo pins
    let trigger_pin = pins.d53.into_output().downgrade();
    let echo_pin = pins.d52.into_floating_input().downgrade();

    // Create SonarSensor instance
    let mut sonar = SonarSensor::new(trigger_pin, echo_pin, timer);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {
        // Get distance and print it
        let distance = sonar.return_distance();
        ufmt::uwriteln!(&mut serial, "Sonar distance: {} cm", distance).void_unwrap();

        // Add some delay before the next measurement
        arduino_hal::delay_ms(16);
    }
}
*/