/*
 * Interrupt Handling Abstraction Example for ATMega2560
 * =============================================
 *
 * This script showcases an abstraction for handling interrupts on the ATMega2560
 * chip using Rust. The abstraction, implemented through the `InterruptController`
 * struct, simplifies the configuration and management of interrupts in a safe and
 * modular way.
 *
 * This script also uses a macro to avoid boilerplate involved with non-blocking delays.
 *
 * Why Abstraction:
 * - Enhances code readability and maintainability by replacing direct register
 *   manipulation with clear, high-level methods.
 * - Promotes safer handling of hardware resources by leveraging Rust's type system.
 * - Facilitates code reuse and scalability for handling multiple types of interrupts.
 */

/*
#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::prelude::*;
use avr_device::interrupt::Mutex;
use core::cell::Cell;

mod tools;
mod hardware;
use tools::millis::{millis, millis_init};
use hardware::peripheral_abstraction::interrupts::{InterruptController, ExternalInterrupt, InterruptMode};

static BLINK_FAST: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[arduino_hal::entry]
fn main() -> ! {
    let mut dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    millis_init(dp.TC0);

    // Set up the LED and button
    let mut led = pins.d13.into_output();
    let _button = pins.d2.into_pull_up_input();

    // Using the interrupt abstraction
    let mut int_controller = InterruptController::new(&mut dp.EXINT);
    int_controller.configure_interrupt(ExternalInterrupt::INT1, InterruptMode::FallingEdge);
    int_controller.enable_interrupt(ExternalInterrupt::INT1);

    // Enable external interrupt for the button
    unsafe { avr_device::interrupt::enable() };

    let mut local_blink_fast = false;
    let mut previous_millis = millis();

    loop {
        let current_millis = millis();

        avr_device::interrupt::free(|cs| {
            local_blink_fast = BLINK_FAST.borrow(cs).get();
        });

        let interval = if local_blink_fast { 100 } else { 1000 };

        execute_after_delay!(current_millis, previous_millis, interval, {
            led.toggle();
        });

    }
}

#[avr_device::interrupt(atmega2560)]
fn INT1() {
    avr_device::interrupt::free(|cs| {
        let current_state = BLINK_FAST.borrow(cs).get();
        BLINK_FAST.borrow(cs).set(!current_state);
    });
}
*/