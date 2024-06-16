use crate::Result;

mod packetserial;

pub use packetserial::{PacketSerial, DEFAULT_ADDRESS};

/// Trait exposing the available methods for controlling the Sabertooth 2x60.
pub trait Sabertooth2x60 {
    // This setting determines how long it takes for the motor driver to shut
    // off if it has not received a command recently. Serial Timeout is off by
    // default. This setting does not persist through a power cycle or in any
    // mode other than packet Serial.
    fn set_serial_timeout(&mut self, ms: u16) -> Result<()>;

    // This value remains until it is changed and does persist through a power
    // cycle. The valid values are:
    // - 2400 baud
    // - 9600 baud (default)
    // - 19200 baud
    // - 38400 baud
    // - 115200 baud
    // This will also update the baud rate of the current serial connection.
    fn set_baud_rate(&mut self, baud_rate: u32) -> Result<()>;

    // This adjusts or disables the ramping feature found on the Sabertooth
    // 2x60. This adjustment applies to all modes, even R/C and analog mode.
    // Lower values mean faster ramping.
    fn set_ramping(&mut self, rate: u8) -> Result<()>;

    // This determines the extent of the Sabertooth's deadband – the range of
    // commands close to "stop" that will be interpreted as stop. This setting
    // applies to all modes and persists through a power cycle.
    fn set_deadband(&mut self, deadband: u8) -> Result<()>;

    // Sets the motor 1 value. -128 is full reverse, 127 is full forward.
    fn drive_m1(&mut self, value: i8) -> Result<()>;

    // Sets the motor 2 value. -128 is full reverse, 127 is full forward.
    fn drive_m2(&mut self, value: i8) -> Result<()>;

    // Sets both motors in mixed mode. -128 is full reverse, 127 is full
    // forward.
    fn drive_mixed(&mut self, value: i8) -> Result<()>;

    // Turns the vehicle in mixed mode. -128 is full left, 127 is full right.
    fn turn_mixed(&mut self, value: i8) -> Result<()>;
}
