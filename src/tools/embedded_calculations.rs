use arduino_hal::simple_pwm::Prescaler;

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