/*!
 * DC Motor Abstraction for Arduino
 * ================================
 *
 * This module provides a `DcMotor` struct, an abstraction for controlling DC motors
 * using Arduino hardware through the `arduino-hal` crate.
 *
 * Features:
 * - The `DcMotor` struct represents a DC motor and allows control over its speed and direction.
 * - Speed control is achieved through PWM (Pulse Width Modulation) on the `en` (enable) pin.
 * - Direction control is managed by setting `in_a` and `in_b` pins high or low.
 *
 * Usage:
 * - The `new` method initializes the motor with specified pins for PWM output and direction control.
 * - `set_speed` adjusts the motor's speed by setting the PWM duty cycle.
 * - `set_direction` changes the motor's rotation direction to either forward or backward.
 * - `turn_on` and `turn_off` methods control the motor's operational state.
 *
 * Parameters:
 * - `TC`: Type associated with the PWM timer.
 * - `E`: PwmPinOps trait, indicating the pin supports PWM operations.
 * - `P1`, `P2`: PinOps trait, indicating the pins support basic I/O operations.
 *
 * Design Consideration:
 * - This abstraction is designed to simplify the control of DC motors by providing a high-level interface.
 * - It encapsulates the lower-level details of pin manipulation, making the motor control code more readable
 *   and easier to maintain.
 *
 * Note:
 * - The `MotorDirection` enum defines the two possible rotation directions: Forward and Backward.
 * - This abstraction is tailored for use with the Arduino ecosystem, specifically with the `arduino-hal` crate.
 */

use arduino_hal::port::{Pin, mode, PinOps};
use arduino_hal::simple_pwm::PwmPinOps;

pub enum MotorDirection {
    Forward,
    Backward,
}

pub struct DcMotor<TC, E, P1, P2> {
    en: Pin<mode::PwmOutput<TC>, E>,
    in_a: Pin<mode::Output, P1>,
    in_b: Pin<mode::Output, P2>,
}

impl<TC, E, P1, P2> DcMotor<TC, E, P1, P2>
    where
        E: PwmPinOps<TC>,
        P1: PinOps,
        P2: PinOps,
{
    pub fn new(
        en: Pin<mode::PwmOutput<TC>, E>,
        in_a: Pin<mode::Output, P1>,
        in_b: Pin<mode::Output, P2>,
    ) -> Self {
        Self { en, in_a, in_b }
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.en.set_duty(speed);
    }

    pub fn set_direction(&mut self, direction: MotorDirection) {
        match direction {
            MotorDirection::Forward => {
                self.in_a.set_high();
                self.in_b.set_low();
            },
            MotorDirection::Backward => {
                self.in_a.set_low();
                self.in_b.set_high();
            },
        }
    }

    pub fn turn_on(&mut self) {
        self.in_a.set_low();
        self.in_b.set_low();
        self.en.enable();
    }

    pub fn turn_off(&mut self) {
        self.in_a.set_low();
        self.in_b.set_low();
        self.en.disable();
    }
}
