use arduino_hal::port::mode::{Floating, Input, Output};
use arduino_hal::port::Pin;
use arduino_hal::hal::port::{Dynamic};
use avr_device::atmega2560::{TC1};
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayUs;

const TRIGGER_UP_TIME: u16 = 10u16;

pub struct SonarSensor {
    trig: Pin<Output, Dynamic>,
    echo: Pin<Input<Floating>, Dynamic>,
    timer: TC1,
}

impl SonarSensor {
    pub fn new(trig: Pin<Output, Dynamic>, echo: Pin<Input<Floating>, Dynamic>, timer: TC1) -> Self {
        Self {
            trig, // trigger a pulse
            echo, // recieve an echo
            timer, // count the time it takes
        }
    }

    pub fn return_distance(&mut self) -> u16 {

        let mut delay = arduino_hal::Delay::new();

        // send out a pulse
        self.trig.set_high();
        delay.delay_us(TRIGGER_UP_TIME);
        self.trig.set_low();

        // wait for echo to go high
        while self.echo.is_low() {

            if self.timer.tcnt1.read().bits() >= 65000 { // if it takes too long, assume no response (target too far)
                return 63500;
            }

        }

        self.timer.tcnt1.write(|w| w.bits(0)); // once we get a pulse, set timer to 0

        while self.echo.is_high() {} // pause while the timer counts up

        (self.timer.tcnt1.read().bits()) / 58 // return time converted to cm
    }
}