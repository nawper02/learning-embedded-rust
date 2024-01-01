#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::simple_pwm::*;

mod tools;

mod hardware;
use hardware::motors::servo::ServoMotor;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // PWM timer.
    // We are prescaling to 1024 to get a frequecy of 61 hz, which is close to the servos
    // 50hz. Because of this, the angle_to_pulse_width method in ServoMotor uses some magic numbers
    // that do not directly correspond to the pulse widths that the servo expects.
    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale1024);

    // Servo PWM pin
    let servo_pwm_pin = pins.d2.into_output().into_pwm(&timer3);

    // Initialize the servo motor with the appropriate pin.
    let mut servo = ServoMotor::new(servo_pwm_pin);

    loop {
        servo.set_position(0.0); // Approximately 0°
        arduino_hal::delay_ms(1000);

        servo.set_position(180.0); // Approximately 180°
        arduino_hal::delay_ms(1000);
    }
}
