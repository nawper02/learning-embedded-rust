/*
/*
  Example Usage of IRSensorArray for Arduino
  -------------------------------------------
  This example demonstrates the usage of the IRSensorArray for both digital and analog sensor modes on an Arduino board.

  Key Components:
  1. Setup for Arduino board using arduino_hal library.
  2. Configuration for both digital and analog sensor arrays using the IRSensorArray.

  Digital Sensor Setup (commented out in this example):
  - Converts digital pins (D2 to D9) into floating inputs and then downgrades them to a generic type.
  - Initializes an IRSensorArray in digital mode with these pins.
  - Includes an example of how to read values from digital sensors (commented out).

  Analog Sensor Setup:
  - Converts analog pins (A0 to A7) into analog inputs and then converts them into channels.
  - Initializes an IRSensorArray in analog mode with these channels.
  - Demonstrates reading values from analog sensors in the main loop.

  Main Loop:
  - For digital sensors (commented out): Reads and prints the state of each digital sensor.
  - For analog sensors: Reads and prints the analog value of each sensor.
  - Includes a delay to prevent flooding the serial output.

  Serial Communication:
  - Sets up serial communication at 57600 baud for debugging and data visualization.

  This example is a practical illustration of how to use the IRSensorArray for different types of sensors on an Arduino. It shows the flexibility of the IRSensorArray in handling multiple sensor types and provides a basis for further development of sensor-based projects.
*/

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::port::mode::{Floating, Input};
use arduino_hal::hal::port::Dynamic;
use arduino_hal::port::Pin;
use arduino_hal::adc::{Adc, Channel};

mod hardware;
use hardware::sensors::ir_array::{IRSensorArray};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = Adc::new(dp.ADC, Default::default());

/*
    // Digital Usage
    let p1 = pins.d2.into_floating_input().downgrade();
    let p2 = pins.d3.into_floating_input().downgrade();
    let p3 = pins.d4.into_floating_input().downgrade();
    let p4 = pins.d5.into_floating_input().downgrade();
    let p5 = pins.d6.into_floating_input().downgrade();
    let p6 = pins.d7.into_floating_input().downgrade();
    let p7 = pins.d8.into_floating_input().downgrade();
    let p8 = pins.d9.into_floating_input().downgrade();

    let ir_pins: [Pin<Input<Floating>, Dynamic>; 8] = [p1,p2,p3,p4,p5,p6,p7,p8];
    let mut ir_array = IRSensorArray::new_digital(ir_pins);
*/


    // Analog Usage
    let p1 = pins.a0.into_analog_input(&mut adc).into_channel();
    let p2 = pins.a1.into_analog_input(&mut adc).into_channel();
    let p3 = pins.a2.into_analog_input(&mut adc).into_channel();
    let p4 = pins.a3.into_analog_input(&mut adc).into_channel();
    let p5 = pins.a4.into_analog_input(&mut adc).into_channel();
    let p6 = pins.a5.into_analog_input(&mut adc).into_channel();
    let p7 = pins.a6.into_analog_input(&mut adc).into_channel();
    let p8 = pins.a7.into_analog_input(&mut adc).into_channel();

    let ir_pins: [Channel<>; 8] = [p1,p2,p3,p4,p5,p6,p7,p8];
    let mut ir_array = IRSensorArray::new_analog(ir_pins);


    loop {
        /*
        // Example of reading from the sensor array in digital mode
        let digital_values = ir_array.digital_read();
        for (i, value) in digital_values.iter().enumerate() {
            ufmt::uwriteln!(&mut serial, "Digital Sensor {}: {}", i, value).void_unwrap();
        }
        */

        // Example of reading from the sensor array in analog mode
        let analog_values = ir_array.analog_read(&mut adc);
        for (i, value) in analog_values.iter().enumerate() {
            ufmt::uwriteln!(&mut serial, "Analog Sensor {}: {}", i, value).void_unwrap();
        }

        arduino_hal::delay_ms(50); // Delay to prevent flooding the serial output
    }
}
*/