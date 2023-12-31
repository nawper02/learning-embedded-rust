/*!
 * Timer Abstraction for ATMega2560
 * =================================
 *
 * This module provides a `Timer` trait to abstract the functionality of timers
 * in the ATMega2560 microcontroller, offering a uniform interface for various timer types.
 *
 * Features:
 * - The `Timer` trait defines common operations such as `prescale`, `read`, `reset`, and `postscale`.
 * - `prescale` configures the timer's prescaler to determine its counting speed.
 * - `read` returns the current timer value as a `u16`, accommodating different timer resolutions.
 * - `reset` zeroes the timer count.
 * - `postscale` stops the timer by removing its clock source.
 *
 * Implementations:
 * - The trait is implemented for `TC0` and `TC1`, representative of 8-bit and 16-bit timers respectively.
 * - Each implementation tailors the trait methods to the specific characteristics of the timer.
 *
 * Usage:
 * - The `Prescaler` enum provides prescaling options to control timer speed.
 *
 * Design Consideration:
 * - While implementing the `Timer` trait for each timer involves some boilerplate, it significantly
 *   cleans up and simplifies timer operations elsewhere in the code.
 * - This abstraction leverages Rust's trait system to ensure type safety and consistency across different timer types.

// ╔═══════════╦══════════════╦═══════════════════╗
// ║ PRESCALER ║ TIMER_COUNTS ║ Overflow Interval ║
// ╠═══════════╬══════════════╬═══════════════════╣
// ║        64 ║          250 ║              1 ms ║
// ║       256 ║          125 ║              2 ms ║
// ║       256 ║          250 ║              4 ms ║
// ║      1024 ║          125 ║              8 ms ║
// ║      1024 ║          250 ║             16 ms ║
// ╚═══════════╩══════════════╩═══════════════════╝

// Page 135 of the 2560 datasheet has info on the 16 bit registers for the other timers.

 */

use avr_device::atmega2560::{TC0, TC1, TC3, TC4, TC5};

pub trait Timer {

    /// Prescales the timer.
    fn prescale(&self, prescaler: Prescaler);

    /// Reads the current timer value.
    fn read(&self) -> u16;

    /// Resets the timer to zero.
    fn reset(&self);

    /// Stops the timer.
    fn postscale(&self);

    /// Sets the timer counts.
    fn set_timer_count(&self, count: u16);

}

// Implementing Timer trait for each timer will be a lot of boilerplate since we must talk to
// each register directly.
// But it is a necessary evil to clean up our code elsewhere.

impl Timer for TC0 {
    fn prescale(&self, prescaler: Prescaler) {
        self.tccr0b.write(|w| match prescaler {
            Prescaler::Prescale8 => w.cs0().prescale_8(),
            Prescaler::Prescale64 => w.cs0().prescale_64(),
            Prescaler::Prescale256 => w.cs0().prescale_256(),
            Prescaler::Prescale1024 => w.cs0().prescale_1024(),
        });
    }

    fn read(&self) -> u16 {
        // it seems the timers have different resolutions.
        // to satisfy the timer traits expected u16 type for the read() method we must safe cast it here.
        self.tcnt0.read().bits() as u16
    }

    fn reset(&self) {
        self.tcnt0.write(|w| w.bits(0));
    }

    fn postscale(&self) {
        self.tccr0b.write(|w| w.cs0().no_clock());
    }

    fn set_timer_count(&self, count: u16) {
        // For an 8-bit timer, we can only use the lower 8 bits of the count
        self.ocr0a.write(|w| w.bits(count as u8));
    }
}

impl Timer for TC1 {
    fn prescale(&self, prescaler: Prescaler) {
        match prescaler {
            Prescaler::Prescale8 => self.tccr1b.write(|w| w.cs1().prescale_8()),
            Prescaler::Prescale64 => self.tccr1b.write(|w| w.cs1().prescale_64()),
            Prescaler::Prescale256 => self.tccr1b.write(|w| w.cs1().prescale_256()),
            Prescaler::Prescale1024 => self.tccr1b.write(|w| w.cs1().prescale_1024()),
        }
    }

    fn read(&self) -> u16 {
        self.tcnt1.read().bits()
    }

    fn reset(&self) {
        self.tcnt1.write(|w| w.bits(0));
    }

    fn postscale(&self) {
        // Removes prescalar.
        self.tccr1b.write(|w| w.cs1().no_clock());
    }

    fn set_timer_count(&self, count: u16) {
        // For a 16-bit timer, we can use the full 16-bit count
        self.icr1.write(|w| w.bits(count));
    }
}

impl Timer for TC3 {
    fn prescale(&self, prescaler: Prescaler) {
        match prescaler {
            Prescaler::Prescale8 => self.tccr3b.write(|w| w.cs3().prescale_8()),
            Prescaler::Prescale64 => self.tccr3b.write(|w| w.cs3().prescale_64()),
            Prescaler::Prescale256 => self.tccr3b.write(|w| w.cs3().prescale_256()),
            Prescaler::Prescale1024 => self.tccr3b.write(|w| w.cs3().prescale_1024()),
        }
    }

    fn read(&self) -> u16 {
        self.tcnt3.read().bits()
    }

    fn reset(&self) {
        self.tcnt3.write(|w| w.bits(0));
    }

    fn postscale(&self) {
        self.tccr3b.write(|w| w.cs3().no_clock());
    }

    fn set_timer_count(&self, count: u16) {
        self.icr3.write(|w| w.bits(count));
    }
}

pub enum Prescaler {
    Prescale8,
    Prescale64,
    Prescale256,
    Prescale1024,
}
