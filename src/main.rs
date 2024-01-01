#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // PWM timer.
    // I believe timer3 is connected to digital pin 2 in the atmega2560.
    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale1024);

    // PWM pin. Controls the position of the motor.
    let mut servo_pin = pins.d2.into_output().into_pwm(&timer3);

    // Enable PWM pin.
    servo_pin.enable();

    loop {
        servo_pin.set_duty(calculate_duty_for_pulse_width(0.4)); // Approximately 0°
        arduino_hal::delay_ms(1000);


        servo_pin.set_duty(calculate_duty_for_pulse_width(3.2)); // Approximately 180°
        arduino_hal::delay_ms(1000);
    }

}

// Calculate the duty cycle for a given pulse width in milliseconds.
fn calculate_duty_for_pulse_width(pulse_width_ms: f32) -> u8 {
    // Total period for 61 Hz PWM frequency (1024 prescaler) in milliseconds.
    let total_period_ms: f32 = 20.0;

    // Maximum duty value for an 8-bit value.
    let max_duty: u8 = 255;

    // Calculate the fraction of the period that the pulse width represents.
    let duty_fraction: f32 = pulse_width_ms / total_period_ms;

    // Convert this fraction to a duty cycle value.
    (duty_fraction * max_duty as f32) as u8
}