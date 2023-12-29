/*
// Example usage of millis.
// Note that millis is hardcoded to use TC0. Doing it generally would be too hard...
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod tools;

use arduino_hal::prelude::*;
use embedded_hal::blocking::delay::DelayMs;
use tools::millis::millis::{millis, millis_init};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // initialize millis
    millis_init(dp.TC0);

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {
        // Get the current milliseconds using the `millis` function
        let time = millis();

        // Print the elapsed milliseconds
        ufmt::uwriteln!(&mut serial, "Elapsed millis: {}", time).void_unwrap();
    }
}

*/
