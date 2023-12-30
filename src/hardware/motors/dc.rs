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
