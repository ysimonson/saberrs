use std::cmp::{max, min};

#[allow(unused_imports)]
use log::debug;

use crate::error::{Error, Result};
use crate::port::SabertoothSerial;
use crate::sabertooth2x60::Sabertooth2x60;
use crate::utils;

#[cfg(feature = "serialport")]
use crate::port::sabertoothport::SabertoothPort;

/// Default address for packet communication.
pub const DEFAULT_ADDRESS: u8 = 128;
pub const MAX_SERIAL_TIMEOUT_MS: u16 = 12700;

/// Interface using the "Packet Serial" protocol.
pub struct PacketSerial<T: SabertoothSerial> {
    dev: T,
    address: u8,
}

#[cfg(feature = "serialport")]
impl PacketSerial<SabertoothPort> {
    /// Open a serial port and return a new instance of `PacketSerial` with
    /// default settings. By default the address is `128` and the protection
    /// type is `PacketType::CRC`.
    ///
    /// # Example
    ///
    /// ```
    /// use saberrs::sabertooth2x60::PacketSerial;
    /// let saber = PacketSerial::new("/dev/ttyUSB0");
    /// ```
    pub fn new(port: &str) -> Result<PacketSerial<SabertoothPort>> {
        Ok(PacketSerial::from(SabertoothPort::new(port)?))
    }
}

impl<T: SabertoothSerial> PacketSerial<T> {
    /// Set the address of the Sabertooth.
    ///
    /// # Example
    ///
    /// ```
    /// use saberrs::sabertooth2x60::PacketSerial;
    /// # use saberrs::{Result, SabertoothPort};
    /// # fn new_saber() -> Result<PacketSerial<SabertoothPort>> {
    /// let saber = PacketSerial::new("/dev/ttyUSB0")?.with_address(129);
    /// # Ok(saber)
    /// # }
    /// ```
    pub fn with_address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    fn write(&mut self, command: u8, data: u8) -> Result<()> {
        let txdata = [
            self.address,
            command,
            data,
            utils::checksum(&[self.address, command, data]),
        ];
        dbg_frame!(tx, txdata);
        Ok(self.dev.write_all(&txdata)?)
    }

    fn write_motor_command(
        &mut self,
        forward_command: u8,
        backward_command: u8,
        value: i8,
    ) -> Result<()> {
        if value >= 0 {
            self.write(forward_command, min(127i8, value) as u8)
        } else {
            self.write(backward_command, (-max(-127i8, value)) as u8)
        }
    }
}

impl<T: SabertoothSerial> From<T> for PacketSerial<T> {
    fn from(dev: T) -> Self {
        PacketSerial {
            dev,
            address: DEFAULT_ADDRESS,
        }
    }
}

impl<T> From<&T> for PacketSerial<T>
where
    T: SabertoothSerial + Clone,
{
    fn from(dev: &T) -> Self {
        PacketSerial {
            dev: dev.clone(),
            address: DEFAULT_ADDRESS,
        }
    }
}

impl<T: SabertoothSerial> Sabertooth2x60 for PacketSerial<T> {
    fn set_serial_timeout(&mut self, ms: u16) -> Result<()> {
        if ms > MAX_SERIAL_TIMEOUT_MS {
            return Err(Error::InvalidInput(format!(
                "timeout must be less than or equal to {MAX_SERIAL_TIMEOUT_MS}"
            )));
        }
        let units = if ms > 0 && ms < 100 { 1 } else { ms / 100 };
        let data = utils::map_range((0, MAX_SERIAL_TIMEOUT_MS), (0, 127), units);
        self.write(14, data as u8)
    }

    fn set_baud_rate(&mut self, baud_rate: u32) -> Result<()> {
        let data = match baud_rate {
            2400 => 1,
            9600 => 2,
            19200 => 3,
            38400 => 4,
            115200 => 5,
            _ => return Err(Error::InvalidInput("invalid baud rate".to_string())),
        };
        self.write(15, data)?;
        self.dev.set_baud_rate(baud_rate)
    }

    fn set_ramping(&mut self, rate: u8) -> Result<()> {
        let data = utils::map_range((0, 255), (0, 80), rate);
        self.write(16, data)
    }

    fn set_deadband(&mut self, deadband: u8) -> Result<()> {
        let data = utils::map_range((0, 255), (0, 127), deadband);
        self.write(17, data)
    }

    fn drive_m1(&mut self, value: i8) -> Result<()> {
        self.write_motor_command(0, 1, value)
    }

    fn drive_m2(&mut self, value: i8) -> Result<()> {
        self.write_motor_command(4, 5, value)
    }

    fn drive_mixed(&mut self, value: i8) -> Result<()> {
        self.write_motor_command(8, 9, value)
    }

    fn turn_mixed(&mut self, value: i8) -> Result<()> {
        self.write_motor_command(10, 11, value)
    }
}
