use arduino_hal::simple_pwm::Prescaler;

pub fn calculate_duty_for_pulse_width(pulse_width_ms: f32) -> u8 {
    // Total period for 50 Hz PWM frequency in milliseconds.
    let total_period_ms: f32 = 20.0;

    // Maximum duty value for an 8-bit value.
    let max_duty: u8 = 255;

    // Calculate the fraction of the period that the pulse width represents.
    let duty_fraction: f32 = pulse_width_ms / total_period_ms;

    // Convert this fraction to a duty cycle value.
    (duty_fraction * max_duty as f32) as u8
}

pub fn calculate_timer_count_for_pwm_frequency(clock_freq: u32, pwm_freq: u32, prescaler: Prescaler) -> u16 {
    let prescaler_value = match prescaler {
        Prescaler::Prescale8 => 8,
        Prescaler::Prescale64 => 64,
        Prescaler::Prescale256 => 256,
        Prescaler::Prescale1024 => 1024,
        _ => 0 // base case needs to be u16
    };

    // Calculate the timer count
    // Formula: Timer Count = (Clock Frequency / (Prescaler * PWM Frequency)) - 1
    (clock_freq / (prescaler_value * pwm_freq)) as u16 - 1
}