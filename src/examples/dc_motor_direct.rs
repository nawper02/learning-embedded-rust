/*
// direct motor control example.
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
    let timer3 = Timer3Pwm::new(dp.TC3, Prescaler::Prescale64);

    // PWM pin. Controls the speed of the motor.
    let mut en_a = pins.d2.into_output().into_pwm(&timer3);

    // Direction control pins (in1 and in2)
    let mut in_1 = pins.d3.into_output();
    let mut in_2 = pins.d4.into_output();

    // Enable PWM pin.
    en_a.enable();

    // Set speed.
    en_a.set_duty(200);

    in_1.set_high();
    in_2.set_low();
    arduino_hal::delay_ms(1000);

    in_1.set_low();
    in_2.set_high();
    arduino_hal::delay_ms(1000);

    for x in (0..=255).chain((0..=254).rev()) {
        en_a.set_duty(x);
        arduino_hal::delay_ms(10);
    }

    in_2.set_low();

    loop {}

}
 */