/* ROUGH SERVO EXAMPLE. BARELY WORKS
#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::simple_pwm::*;

mod tools;
use tools::embedded_calculations::calculate_duty_for_pulse_width;


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

 */