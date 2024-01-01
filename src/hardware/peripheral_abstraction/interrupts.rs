/*!
 * InterruptController Module for ATMega2560
 * =========================================
 *
 * This module provides an `InterruptController` struct for the ATMega2560 microcontroller,
 * facilitating easier and safer management of various types of interrupts in Rust.
 *
 * Overview:
 * - The `InterruptController` struct holds a mutable reference to the `EXINT` (External Interrupt)
 *   system of the ATMega2560.
 * - It provides methods to configure, enable, and disable external interrupts and pin change interrupts,
 *   abstracting away direct register manipulation.
 *
 * Usage:
 * - `InterruptController::new` creates a new instance with a reference to `EXINT`.
 * - `configure_interrupt` method sets up an external interrupt (INT0 to INT7) with a specified mode
 *   (e.g., FallingEdge, RisingEdge). It configures the interrupt sense control bits for the specified
 *   interrupt in the EICRA (External Interrupt Control Register A) or EICRB register.
 * - `enable_interrupt` method enables a specific external interrupt in the EIMSK (External Interrupt
 *   Mask Register), allowing the microcontroller to respond to the interrupt.
 * - `disable_interrupt` method (if implemented) disables a specific external interrupt.
 * - `enable_pcint` and `disable_pcint` methods control the pin change interrupts (PCINT0, PCINT1, PCINT2).
 *   They use exint.pcicr.modify() to set the enabled interrupts. I got a bit confused and
 *   I don't know if it works the way I wrote it. I mainly just dont know if I set the closure
 *   up right: w.pcie().bits(). Maybe its w.bits(). Idk.
 *   These methods are marked as deprecated since they are untested and require thorough testing before use.
 *
 * Detailed Explanation:
 * - The `configure_interrupt` method modifies EICRA or EICRB, setting the interrupt sense control bits
 *   (ISCn) for a specific external interrupt (INTn). This determines the condition (like rising or falling edge)
 *   that triggers the interrupt.
 * - The `enable_interrupt` method modifies the EIMSK, setting the bit that corresponds to a specified external
 *   interrupt. This enables the microcontroller to respond to that interrupt. It uses bitwise operations to ensure
 *   only the specified interrupt is enabled, without affecting other bits/interrupts in EIMSK.
 *
 * Design:
 * - The `ExternalInterrupt` enum represents the available external interrupts (INT0 to INT7).
 * - The `PinChangeInterrupt` enum represents the available pin change interrupts (PCINT0 to PCINT2).
 * - The `InterruptMode` enum defines the possible triggering modes for external interrupts.
 * - This abstraction leverages Rust's strong type system to reduce errors and enhance code clarity.
 *
 * Note:
 * - This module is specific to the ATMega2560 and is designed to be used with the `avr_device` crate.
 * - It provides a higher-level interface for interrupt configuration, suited for embedded applications
 *   where safety and simplicity are paramount.
 * - The `enable_pcint` and `disable_pcint` methods are currently marked as deprecated due to their untested status.
 */


use avr_device::atmega2560::EXINT;

pub struct InterruptController<'a> {
    exint: &'a mut EXINT,
}

impl<'a> InterruptController<'a> {
    pub fn new(exint: &'a mut EXINT) -> Self {
        InterruptController { exint }
    }

    pub fn configure_interrupt(&mut self, int: ExternalInterrupt, mode: InterruptMode) {
        match int {
            ExternalInterrupt::INT0 => {
                self.exint.eicra.modify(|_, w| w.isc0().bits(mode as u8));
            },
            ExternalInterrupt::INT1 => {
                self.exint.eicra.modify(|_, w| w.isc1().bits(mode as u8));
            },
            ExternalInterrupt::INT2 => {
                self.exint.eicra.modify(|_, w| w.isc2().bits(mode as u8));
            },
            ExternalInterrupt::INT3 => {
                self.exint.eicra.modify(|_, w| w.isc3().bits(mode as u8));
            },
            ExternalInterrupt::INT4 => {
                self.exint.eicrb.modify(|_, w| w.isc4().bits(mode as u8));
            },
            ExternalInterrupt::INT5 => {
                self.exint.eicrb.modify(|_, w| w.isc5().bits(mode as u8));
            },
            ExternalInterrupt::INT6 => {
                self.exint.eicrb.modify(|_, w| w.isc6().bits(mode as u8));
            },
            ExternalInterrupt::INT7 => {
                self.exint.eicrb.modify(|_, w| w.isc7().bits(mode as u8));
            },
            _ => {}
        }
    }

    pub fn enable_interrupt(&mut self, int: ExternalInterrupt) {
        // Enable the specified interrupt
        self.exint.eimsk.modify(|_, w| w.bits(1 << (int as u8)) );
    }

    #[deprecated(note = "Untested code - needs thorough testing before use")]
    pub fn enable_pcint(&mut self, pcint: PinChangeInterrupt) {
        let mask = match pcint {
            PinChangeInterrupt::PCINT0 => 0b001,
            PinChangeInterrupt::PCINT1 => 0b010,
            PinChangeInterrupt::PCINT2 => 0b100,
        };
        self.exint.pcicr.modify(|_, w| w.pcie().bits(mask) );
    }

    #[deprecated(note = "Untested code - needs thorough testing before use")]
    pub fn disable_pcint(&mut self, pcint: PinChangeInterrupt) {
        let mask = match pcint {
            PinChangeInterrupt::PCINT0 => !0b001,
            PinChangeInterrupt::PCINT1 => !0b010,
            PinChangeInterrupt::PCINT2 => !0b100,
        };
        self.exint.pcicr.modify(|_, w| w.pcie().bits(mask) );
    }
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
}

pub enum PinChangeInterrupt {
    PCINT0,
    PCINT1,
    PCINT2,
}

pub enum InterruptMode {
    LowLevel = 0x00,
    AnyChange = 0x01,
    FallingEdge = 0x02,
    RisingEdge = 0x03,
}
