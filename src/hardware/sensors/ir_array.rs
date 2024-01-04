/*
  IRSensorArray in Rust using arduino_hal library
  ------------------------------------------------
  This IRSensorArray is designed for Arduino boards, capable of handling both digital and analog sensors.

  Understanding `Channel` and `Dynamic`:
  1. `Channel` in `into_channel()`:
     - Represents an analog input channel on Arduino.
     - Used to convert a generic pin into an analog channel for analog input.
     - In `new_analog`, `[Channel; 8]` allows managing up to 8 analog inputs.

  2. `Dynamic` in `pin.downgrade()`:
     - Provides flexibility in handling various pin types.
     - `downgrade()` method converts a specific pin type into a generic `Dynamic` type.
     - Useful for storing various types of pins uniformly, as seen in `SensorArray::DigitalPins`.

  Setup for Analog and Digital Values:
  1. Analog Setup:
     - Connect analog sensors to Arduino's analog input pins.
     - `new_analog` takes `[Channel; 8]` for analog input channels.
     - `analog_read` uses `adc.read_blocking(channel)` to read analog values.

  2. Digital Setup:
     - Connect digital sensors to Arduino's digital pins.
     - `new_digital` takes `[Pin<Input<Floating>, Dynamic>; 8]` for digital input pins.
     - `digital_read` checks the state of each digital pin with `pin.is_high()`.

  Switching Between Modes:
  - `IRSensorArray` can operate in either `SensorMode::Analog` or `SensorMode::Digital`.
  - `set_mode` method allows switching between analog and digital modes.
  - Assertions in `digital_read` and `analog_read` ensure the sensor array is in the correct mode before reading.

  This design allows for flexible use of the sensor array for various sensor inputs in Arduino projects, adaptable for both analog and digital sensor types.
*/

use arduino_hal::adc::{Adc, Channel};
use arduino_hal::port::mode::{Input, Analog, Output, Floating};
use arduino_hal::port::Pin;
use arduino_hal::hal::port::Dynamic;
use embedded_hal::digital::v2::InputPin;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum SensorMode {
    Digital,
    Analog,
}

enum SensorArray {
    DigitalPins([Pin<Input<Floating>, Dynamic>; 8]),
    AnalogChannels([Channel; 8]),
}

pub struct IRSensorArray {
    sensors: SensorArray,
    mode: SensorMode,
}

impl IRSensorArray {
    pub fn new_digital(pins: [Pin<Input<Floating>, Dynamic>; 8]) -> Self {
        Self {
            sensors: SensorArray::DigitalPins(pins),
            mode: SensorMode::Digital,
        }
    }

    pub fn new_analog(channels: [Channel; 8]) -> Self {
        Self {
            sensors: SensorArray::AnalogChannels(channels),
            mode: SensorMode::Analog,
        }
    }

    pub fn set_mode(&mut self, mode: SensorMode) {
        self.mode = mode;
    }

    pub fn digital_read(&self) -> [bool; 8] {
        assert_eq!(self.mode, SensorMode::Digital, "Sensor is not in Digital mode");
        if let SensorArray::DigitalPins(pins) = &self.sensors {
            let mut values = [false; 8];
            for (i, pin) in pins.iter().enumerate() {
                values[i] = pin.is_high();
            }
            values
        } else {
            panic!("Sensor array is not configured with digital pins");
        }
    }

    pub fn analog_read(&self, adc: &mut Adc) -> [u16; 8] {
        assert_eq!(self.mode, SensorMode::Analog, "Sensor is not in Analog mode");
        if let SensorArray::AnalogChannels(channels) = &self.sensors {
            let mut values = [0u16; 8];
            for (i, channel) in channels.iter().enumerate() {
                values[i] = adc.read_blocking(channel);
            }
            values
        } else {
            panic!("Sensor array is not configured with analog channels");
        }
    }
}