/*
// Example usage of analog_read().
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // get the analog-to-digital converter. it changes the analog values into readable numbers.
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // get analog pin 7
    let a7 = pins.a7.into_analog_input(&mut adc);

    loop {
        // Get the value of the encoder
        let value = a7.analog_read(&mut adc);

        // Print the elapsed milliseconds
        ufmt::uwriteln!(&mut serial, "Analog read value: {}", value).void_unwrap();

        // add some delay
        arduino_hal::delay_ms(16);
    }
}
*/