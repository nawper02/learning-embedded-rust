#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::simple_pwm::*;
use arduino_hal::hal::port::PinOps;
use arduino_hal::simple_pwm::PwmPinOps;

mod hardware;
use hardware::motors::dc::{DcMotor, MotorDirection};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // PWM timer.
    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    // Initialize the motor with the appropriate pins.
    let mut motor = DcMotor::new(
        pins.d2.into_output().into_pwm(&timer3),
        pins.d3.into_output(),
        pins.d4.into_output(),
    );

    // Turn on the motor.
    motor.turn_on();

    // Set speed and direction.
    motor.set_speed(200);
    motor.set_direction(MotorDirection::Forward);
    arduino_hal::delay_ms(1000);

    motor.set_direction(MotorDirection::Backward);
    arduino_hal::delay_ms(1000);

    // Vary speed.
    for x in (0..=255).chain((0..=254).rev()) {
        motor.set_speed(x);
        arduino_hal::delay_ms(10);
    }

    // Turn off the motor.
    motor.turn_off();

    loop {}
}
