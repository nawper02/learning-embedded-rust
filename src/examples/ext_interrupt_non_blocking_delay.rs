/*
 * Arduino Mega Non-blocking LED Blink with External Interrupt
 *
 * This Rust program is designed to run on an Arduino Mega and demonstrates
 * non-blocking LED blinking with the ability to toggle the blink speed using
 * an external interrupt. The main features of this program include:
 *
 * 1. Non-Blocking LED Blinking:
 *    - Utilizes a custom `millis` function to keep track of elapsed time,
 *      allowing the LED to blink without using blocking `delay` calls.
 *    - The LED on pin 13 toggles between two states (ON/OFF) at a specified
 *      interval, which changes upon an external interrupt.
 *
 * 2. External Interrupt Handling:
 *    - An external interrupt is set up on INT1 (digital pin 20) of the
 *      Arduino Mega.
 *    - The interrupt toggles the blink rate of the LED between fast and slow
 *      upon a button press.
 *    - Uses Rust's `avr_device::interrupt` module for interrupt handling.
 *
 * 3. Use of Mutex<Cell<bool>> for Shared State Management:
 *    - A global static variable `BLINK_FAST` is used to store the state of
 *      the blinking speed (fast or slow).
 *    - Mutex and Cell are used to safely modify `BLINK_FAST` in the interrupt
 *      context and read it in the main loop.
 *    - This ensures thread-safe access to shared data.
 *
 * 4. millis Function:
 *    - The `millis` function provides a millisecond counter using hardware
 *      timer TC0.
 *    - This function is essential for the non-blocking behavior of the LED
 *      blinking.
 *    - It's initialized in the main function and then used to track elapsed
 *      time for toggling the LED.
 *
 * NOTES
 * Interrupt Configuration:
 *    - The code configures an external interrupt INT1 (which corresponds to
 *      digital pin 20 on the Arduino Mega) to detect a falling edge signal.
 *      Falling edge means the interrupt is triggered when the signal goes from
 *      HIGH to LOW (e.g., button press).
 *    - `dp.EXINT.eicra.modify(|_, w| w.isc1().bits(0x02));`
 *        - This line configures the External Interrupt Control Register A (EICRA).
 *        - `isc1().bits(0x02)` sets the Interrupt Sense Control for INT1 to detect
 *          a falling edge. `0x02` represents the falling edge configuration.
 *          For a rising edge, you would use `0x03`.
 *    - `dp.EXINT.eimsk.modify(|_, w| w.int().bits(0b00000010));`
 *        - This line enables the external interrupt mask for INT1.
 *        - `eimsk` stands for External Interrupt Mask Register.
 *        - The binary `0b00000010` specifically enables only INT1 and leaves
 *          other external interrupts disabled.
 *
 * Use of wrapping_sub for Millis Calculation:
 *    - `wrapping_sub` is used for safe arithmetic subtraction where overflow
 *      can occur due to the limited range of counters (like `u32`).
 *    - In the context of a millis counter, `wrapping_sub` handles the scenario
 *      where the counter overflows and resets back to zero.
 *    - For example, if the counter is near its maximum value and then overflows,
 *      a standard subtraction would result in incorrect (possibly negative) values.
 *    - `wrapping_sub` allows the subtraction to "wrap around" the maximum value,
 *      correctly calculating the time difference even across an overflow boundary.
 *    - This is crucial for accurately measuring elapsed time with a continually
 *      incrementing counter like `millis`.
 *
 * AVAILABLE INTERRUPTS
    RESET,
    INT0,
    INT1,
    INT2,
    INT3,
    INT4,
    INT5,
    INT6,
    INT7,
    PCINT0,
    PCINT1,
    PCINT2,
    WDT,
    TIMER2_COMPA,
    TIMER2_COMPB,
    TIMER2_OVF,
    TIMER1_CAPT,
    TIMER1_COMPA,
    TIMER1_COMPB,
    TIMER1_COMPC,
    TIMER1_OVF,
    TIMER0_COMPA,
    TIMER0_COMPB,
    TIMER0_OVF,
    SPI_STC,
    USART0_RX,
    USART0_UDRE,
    USART0_TX,
    ANALOG_COMP,
    ADC,
    EE_READY,
    TIMER3_CAPT,
    TIMER3_COMPA,
    TIMER3_COMPB,
    TIMER3_COMPC,
    TIMER3_OVF,
    USART1_RX,
    USART1_UDRE,
    USART1_TX,
    TWI,
    SPM_READY,
    TIMER4_CAPT,
    TIMER4_COMPA,
    TIMER4_COMPB,
    TIMER4_COMPC,
    TIMER4_OVF,
    TIMER5_CAPT,
    TIMER5_COMPA,
    TIMER5_COMPB,
    TIMER5_COMPC,
    TIMER5_OVF,
    USART2_RX,
    USART2_UDRE,
    USART2_TX,
    USART3_RX,
    USART3_UDRE,
    USART3_TX,
 */

/* Start of code

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::prelude::*;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

mod tools;
use tools::millis::millis::{millis, millis_init};

static BLINK_FAST: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    millis_init(dp.TC0);

    // Set up the LED and button
    let mut led = pins.d13.into_output();
    let _button = pins.d2.into_pull_up_input();

    // Configure interrupt
    // Configure INT1 for falling edge. 0x03 would be rising edge.
    // w.isc1: isc1 is the sense control bit for interrupt 1.
    // the EICRA register controls the sense for INT0 through INT3.
    dp.EXINT.eicra.modify(|_, w| w.isc1().bits(0x02));
    // Enable the INT1 interrupt source. this bit for INT1.
    dp.EXINT.eimsk.modify(|_, w| w.int().bits(0b00000010));

    // Enable external interrupt for the button
    unsafe { avr_device::interrupt::enable() };

    let mut local_blink_fast = false;

    // Variable to store the last time the LED was toggled
    let mut previous_millis = millis();

    loop {
        // Get the current time
        let current_millis = millis();

        // Critical section to read the shared state
        avr_device::interrupt::free(|cs| {
            local_blink_fast = BLINK_FAST.borrow(cs).get();
        });

        // Determine the appropriate blink interval
        let interval = if local_blink_fast { 100 } else { 1000 };

        // Check if it's time to toggle the LED
        if current_millis.wrapping_sub(previous_millis) >= interval {
            previous_millis = current_millis;
            led.toggle();
        }
    }
}

#[avr_device::interrupt(atmega2560)]
fn INT1() {
    avr_device::interrupt::free(|cs| {
        // Critical Section: access to BLINK_FAST is locked in here
        let current_state = BLINK_FAST.borrow(cs).get();
        BLINK_FAST.borrow(cs).set(!current_state);
    });
}
*/
