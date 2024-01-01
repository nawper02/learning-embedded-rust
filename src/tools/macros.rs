/**
 * `execute_after_delay` Macro
 * ----------------------------
 *
 * Purpose:
 * Executes a specified action (closure, function call, or code block) after a delay interval has passed.
 * This macro is designed for use in non-blocking scenarios, particularly in embedded systems where
 * maintaining a responsive system is crucial.
 *
 * Usage:
 * `execute_after_delay!(current_millis, previous_millis, interval, action);`
 *
 * Parameters:
 * - `current_millis`: A value representing the current time in milliseconds. Typically obtained from a
 *   function like `millis()`.
 * - `previous_millis`: A mutable reference to a variable that tracks the time (in milliseconds) when the
 *   action was last executed. This variable will be updated to `current_millis` after the action is executed.
 * - `interval`: The delay interval (in milliseconds) after which the action should be executed.
 * - `action`: The action to be executed after the delay. This can be a closure, a function call, or a
 *   block of code.
 *
 * How It Works:
 * The macro checks if the difference between `current_millis` and `previous_millis` is greater than or
 * equal to the `interval`. If so, it updates `previous_millis` to `current_millis` and executes the
 * specified `action`.
 *
 * Example:
 * ```rust
    // Using an inline code block
    execute_after_delay!(current_millis, previous_millis, interval, { led.toggle(); });

    // Using a closure
    let toggle_led = || { led.toggle(); };
    execute_after_delay!(current_millis, previous_millis, interval, toggle_led);

    // Using a function call
    fn toggle_led() {
        led.toggle();
    }
    execute_after_delay!(current_millis, previous_millis, interval, toggle_led());

    // Using a single expression
    execute_after_delay!(current_millis, previous_millis, interval, led.toggle());
 * ```
 *
 * Note:
 * This macro is useful for implementing periodic actions in a non-blocking manner, allowing for
 * other tasks to run concurrently without being interrupted by delay or sleep functions.
 */
#[macro_export]
macro_rules! delayed_execute_nb {
    ($current_millis:expr, $previous_millis:expr, $interval:expr, $action:expr) => {
        if $current_millis.wrapping_sub($previous_millis) >= $interval {
            $previous_millis = $current_millis;
            $action();
        }
    };
}

