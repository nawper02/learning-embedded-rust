#[macro_export]
macro_rules! execute_after_delay {
    ($current_millis:expr, $previous_millis:expr, $interval:expr, $action:block) => {
        if $current_millis.wrapping_sub($previous_millis) >= $interval {
            $previous_millis = $current_millis;
            $action
        }
    };
}
