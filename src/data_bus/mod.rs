mod eightbit_bus;
mod fourbit_bus;
mod i2c_bus;
mod fourbit_eightbit_bus;

use embedded_hal::delay::DelayNs;
pub use self::eightbit_bus::EightBitBus;
pub use self::fourbit_bus::FourBitBus;
pub use self::i2c_bus::I2CBus;

use crate::error::Result;

/// A trait for LCD display buses.
pub trait DataBus {
    /// Sends a command to the display.
    /// `byte`: The command to send.
    /// `delay`: A delay provider.
    /// `data`: Whether the command is data or a command.
    /// Returns: `Ok(())` if the command was sent successfully, `Err(Error)` otherwise.
    fn write<D: DelayNs>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()>;
}
