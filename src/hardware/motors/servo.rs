use arduino_hal::port::{Pin, mode};
use arduino_hal::simple_pwm::PwmPinOps;

pub struct ServoMotor<TC, E> {
    servo_pin: Pin<mode::PwmOutput<TC>, E>,
}

impl<TC, E> ServoMotor<TC, E>
    where
        E: PwmPinOps<TC>,
{
    pub fn new(
        mut servo_pin: Pin<mode::PwmOutput<TC>, E>,
    ) -> Self {
        servo_pin.enable();
        Self { servo_pin }
    }

    pub fn set_position(&mut self, angle: f32) {
        let pulse_width = self.angle_to_pulse_width(angle);
        let duty = ServoMotor::<TC, E>::calculate_duty_for_pulse_width(pulse_width);
        self.servo_pin.set_duty(duty);
    }

    fn angle_to_pulse_width(&self, angle: f32) -> f32 {
        // Convert the angle to a pulse width.
        // Adjust this formula according to your servo's specifications.
        0.5 + (angle / 180.0) * 2.0
    }

    fn calculate_duty_for_pulse_width(pulse_width_ms: f32) -> u8 {
        // Total period for 50 Hz PWM frequency in milliseconds.
        let total_period_ms: f32 = 20.0;

        // Maximum duty value for an 8-bit value.
        let max_duty: u8 = 255;

        // Calculate the fraction of the period that the pulse width represents.
        let duty_fraction: f32 = pulse_width_ms / total_period_ms;

        // Convert this fraction to a duty cycle value.
        (duty_fraction * max_duty as f32) as u8
    }
}
