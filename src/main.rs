#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)] // for millis

mod hardware;
mod tools;

use arduino_hal::prelude::*;
use panic_halt as _;
use embedded_hal::serial::Read;
use hardware::sensors::{echo_module::SonarSensor};
use tools::millis::millis::{millis, millis_init};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Heating up...\r").void_unwrap();

    // LED pin
    let mut led = pins.d31.into_output();

    // Sensor Pins
    let trig = pins.d30.into_output().downgrade();
    let echo = pins.d32.into_floating_input().downgrade();

    // Sensor timer and sonar object
    let sensor_timer = dp.TC1;
    sensor_timer.tccr1b.write(|w| w.cs1().prescale_64());
    let mut sonar = SonarSensor::new(trig, echo, sensor_timer);

    // millis
    millis_init(dp.TC0); // Initialize millis

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    loop {
        // Read distance
        //let distance = sonar.return_distance();
        // Toggle LED
        //led.toggle();

        //arduino_hal::delay_ms(10*distance);

        //ufmt::uwriteln!(&mut serial, "Sonar distance: {} cm", distance).void_unwrap();

        let b = nb::block!(serial.read()).void_unwrap();

        let time = millis();
        ufmt::uwriteln!(&mut serial, "Got {} after {} ms!\r", b, time).void_unwrap();
    }
}
