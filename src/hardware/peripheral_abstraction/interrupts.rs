/*!
 * InterruptController Module for ATMega2560
 * =========================================
 *
 * This module provides an `InterruptController` struct for the ATMega2560 microcontroller,
 * facilitating easier and safer management of external interrupts in Rust.
 *
 * Overview:
 * - The `InterruptController` struct holds a mutable reference to the `EXINT` (External Interrupt)
 *   system of the ATMega2560.
 * - It provides methods to configure and enable interrupts, abstracting away direct register manipulation.
 *
 * Usage:
 * - `InterruptController::new` creates a new instance with a reference to `EXINT`.
 * - `configure_interrupt` method sets up an interrupt (INT0 to INT7) with a specified mode
 *   (e.g., FallingEdge, RisingEdge).
 * - `enable_interrupt` method enables the specified interrupt.
 *
 * Design:
 * - The `ExternalInterrupt` enum represents the available external interrupts (INT0 to INT7).
 * - The `InterruptMode` enum defines the possible triggering modes for these interrupts.
 * - This abstraction leverages Rust's strong type system to reduce errors and enhance code clarity.
 *
 * Note:
 * - This module is specific to the ATMega2560 and is designed to be used with the `avr_device` crate.
 * - It provides a higher-level interface for interrupt configuration, suited for embedded applications
 *   where safety and simplicity are paramount.
 */

use avr_device::atmega2560::EXINT;

pub struct InterruptController<'a> {
    exint: &'a mut EXINT,
    // Other fields as needed
}

impl<'a> InterruptController<'a> {
    pub fn new(exint: &'a mut EXINT) -> Self {
        InterruptController { exint }
    }

    pub fn configure_interrupt(&mut self, int: ExternalInterrupt, mode: InterruptMode) {
        match int {
            ExternalInterrupt::INT0 => {
                // Configure INT0
                self.exint.eicra.modify(|_, w| w.isc0().bits(mode as u8));
            },
            ExternalInterrupt::INT1 => {
                // Configure INT1
                self.exint.eicra.modify(|_, w| w.isc1().bits(mode as u8));
            },
            // ... handle other interrupts
            _ => {}
        }
    }

    pub fn enable_interrupt(&mut self, int: ExternalInterrupt) {
        // Enable the specified interrupt
        self.exint.eimsk.modify(|_, w| unsafe { w.bits(1 << (int as u8)) });
    }

    // Other methods as needed
}

pub enum ExternalInterrupt {
    INT0,
    INT1,
    INT2,
    INT3,
    INT4,
    INT5,
    INT6,
    INT7,
    // Add pin change interrupts if needed
}

pub enum InterruptMode {
    LowLevel = 0x00,
    AnyChange = 0x01,
    FallingEdge = 0x02,
    RisingEdge = 0x03,
    // Add other modes as needed
}
