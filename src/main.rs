// direct motor control example.
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

use arduino_hal::simple_pwm::*;
mod tools;
mod hardware;
use tools::embedded_calculations::{calculate_duty_for_pulse_width, calculate_timer_count_for_pwm_frequency};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Configure TC3.
    let clock_freq = 16_000_000; // 16 MHz for Arduino boards
    let pwm_freq = 50; // 50 Hz for servo control
    let timer_count = calculate_timer_count_for_pwm_frequency(clock_freq, pwm_freq, Prescaler::Prescale64);

    // Set timer count
    dp.TC3.icr3.write(|w| w.bits(timer_count));

    // PWM timer.
    // I believe timer3 is connected to digital pin 2 in the atmega2560.
    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    // PWM pin. Controls the position of the motor.
    let mut servo_pin = pins.d2.into_output().into_pwm(&timer3);

    // Enable PWM pin.
    servo_pin.enable();

    loop {
        servo_pin.set_duty(calculate_duty_for_pulse_width(1.0)); // Approximately 0°
        arduino_hal::delay_ms(1000);


        servo_pin.set_duty(calculate_duty_for_pulse_width(2.0)); // Approximately 180°
        arduino_hal::delay_ms(1000);
    }

}
