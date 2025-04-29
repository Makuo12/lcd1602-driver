#![allow(warnings)]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

use crate::data_bus::DataBus;
use crate::error::{Error, Result};

/// An enum representing different bus configurations.
pub enum BusWidth<
    RS: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    FourBitBus(FourBitBus<RS, EN, D4, D5, D6, D7>),
    EightBitBus(EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>),
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > BusWidth<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    /// Creates a new `BusWidth` instance with the 4-bit bus configuration.
    pub fn new_four_bit(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> BusWidth<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
        let four_bit_bus = FourBitBus::from_pins(rs, en, d4, d5, d6, d7);
        BusWidth::FourBitBus(four_bit_bus)
    }

    /// Creates a new `BusWidth` instance with the 8-bit bus configuration.
    pub fn new_eight_bit(
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> BusWidth<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
        let eight_bit_bus = EightBitBus::from_pins(rs, en, d0, d1, d2, d3, d4, d5, d6, d7);
        BusWidth::EightBitBus(eight_bit_bus)
    }
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > DataBus for BusWidth<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    fn write<D: DelayNs>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()> {
        match self {
            BusWidth::FourBitBus(bus) => bus.write(byte, data, delay),
            BusWidth::EightBitBus(bus) => bus.write(byte, data, delay),
        }
    }
}

/// A struct for 4-bit bus communication.
pub struct FourBitBus<
    RS: OutputPin,
    EN: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    rs: RS,
    en: EN,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin>
    FourBitBus<RS, EN, D4, D5, D6, D7>
{
    /// Creates a new `FourBitBus` instance.
    pub fn from_pins(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> FourBitBus<RS, EN, D4, D5, D6, D7> {
        FourBitBus {
            rs,
            en,
            d4,
            d5,
            d6,
            d7,
        }
    }

    fn write_lower_nibble(&mut self, data: u8) -> Result<()> {
        let db0: bool = (0b0000_0001 & data) != 0;
        let db1: bool = (0b0000_0010 & data) != 0;
        let db2: bool = (0b0000_0100 & data) != 0;
        let db3: bool = (0b0000_1000 & data) != 0;

        if db0 {
            self.d4.set_high().map_err(|_| Error)?;
        } else {
            self.d4.set_low().map_err(|_| Error)?;
        }

        if db1 {
            self.d5.set_high().map_err(|_| Error)?;
        } else {
            self.d5.set_low().map_err(|_| Error)?;
        }

        if db2 {
            self.d6.set_high().map_err(|_| Error)?;
        } else {
            self.d6.set_low().map_err(|_| Error)?;
        }

        if db3 {
            self.d7.set_high().map_err(|_| Error)?;
        } else {
            self.d7.set_low().map_err(|_| Error)?;
        }

        Ok(())
    }

    fn write_upper_nibble(&mut self, data: u8) -> Result<()> {
        let db4: bool = (0b0001_0000 & data) != 0;
        let db5: bool = (0b0010_0000 & data) != 0;
        let db6: bool = (0b0100_0000 & data) != 0;
        let db7: bool = (0b1000_0000 & data) != 0;

        if db4 {
            self.d4.set_high().map_err(|_| Error)?;
        } else {
            self.d4.set_low().map_err(|_| Error)?;
        }

        if db5 {
            self.d5.set_high().map_err(|_| Error)?;
        } else {
            self.d5.set_low().map_err(|_| Error)?;
        }

        if db6 {
            self.d6.set_high().map_err(|_| Error)?;
        } else {
            self.d6.set_low().map_err(|_| Error)?;
        }

        if db7 {
            self.d7.set_high().map_err(|_| Error)?;
        } else {
            self.d7.set_low().map_err(|_| Error)?;
        }
        Ok(())
    }
}

impl<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin>
    DataBus for FourBitBus<RS, EN, D4, D5, D6, D7>
{
    fn write<D: DelayNs>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()> {
        if data {
            self.rs.set_high().map_err(|_| Error)?;
        } else {
            self.rs.set_low().map_err(|_| Error)?;
        }

        self.write_upper_nibble(byte)?;

        // Pulse the enable pin to receive the upper nibble
        self.en.set_high().map_err(|_| Error)?;
        delay.delay_ms(2u32);
        self.en.set_low().map_err(|_| Error)?;

        self.write_lower_nibble(byte)?;

        // Pulse the enable pin to receive the lower nibble
        self.en.set_high().map_err(|_| Error)?;
        delay.delay_ms(2u32);
        self.en.set_low().map_err(|_| Error)?;

        if data {
            self.rs.set_low().map_err(|_| Error)?;
        }
        Ok(())
    }
}

/// A struct for 8-bit bus communication.
pub struct EightBitBus<
    RS: OutputPin,
    EN: OutputPin,
    D0: OutputPin,
    D1: OutputPin,
    D2: OutputPin,
    D3: OutputPin,
    D4: OutputPin,
    D5: OutputPin,
    D6: OutputPin,
    D7: OutputPin,
> {
    rs: RS,
    en: EN,
    d0: D0,
    d1: D1,
    d2: D2,
    d3: D3,
    d4: D4,
    d5: D5,
    d6: D6,
    d7: D7,
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    /// Creates a new `EightBitBus` instance.
    pub fn from_pins(
        rs: RS,
        en: EN,
        d0: D0,
        d1: D1,
        d2: D2,
        d3: D3,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
    ) -> EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7> {
        EightBitBus {
            rs,
            en,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
            d6,
            d7,
        }
    }
}

impl<
        RS: OutputPin,
        EN: OutputPin,
        D0: OutputPin,
        D1: OutputPin,
        D2: OutputPin,
        D3: OutputPin,
        D4: OutputPin,
        D5: OutputPin,
        D6: OutputPin,
        D7: OutputPin,
    > DataBus for EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>
{
    fn write<D: DelayNs>(
        &mut self,
        byte: u8,
        data: bool,
        delay: &mut D,
    ) -> Result<()> {
        if data {
            self.rs.set_high().map_err(|_| Error)?;
        } else {
            self.rs.set_low().map_err(|_| Error)?;
        }

        let db0: bool = (0b0000_0001 & byte) != 0;
        let db1: bool = (0b0000_0010 & byte) != 0;
        let db2: bool = (0b0000_0100 & byte) != 0;
        let db3: bool = (0b0000_1000 & byte) != 0;
        let db4: bool = (0b0001_0000 & byte) != 0;
        let db5: bool = (0b0010_0000 & byte) != 0;
        let db6: bool = (0b0100_0000 & byte) != 0;
        let db7: bool = (0b1000_0000 & byte) != 0;

        if db0 {
            self.d0.set_high().map_err(|_| Error)?;
        } else {
            self.d0.set_low().map_err(|_| Error)?;
        }

        if db1 {
            self.d1.set_high().map_err(|_| Error)?;
        } else {
            self.d1.set_low().map_err(|_| Error)?;
        }

        if db2 {
            self.d2.set_high().map_err(|_| Error)?;
        } else {
            self.d2.set_low().map_err(|_| Error)?;
        }

        if db3 {
            self.d3.set_high().map_err(|_| Error)?;
        } else {
            self.d3.set_low().map_err(|_| Error)?;
        }

        if db4 {
            self.d4.set_high().map_err(|_| Error)?;
        } else {
            self.d4.set_low().map_err(|_| Error)?;
        }

        if db5 {
            self.d5.set_high().map_err(|_| Error)?;
        } else {
            self.d5.set_low().map_err(|_| Error)?;
        }

        if db6 {
            self.d6.set_high().map_err(|_| Error)?;
        } else {
            self.d6.set_low().map_err(|_| Error)?;
        }

        if db7 {
            self.d7.set_high().map_err(|_| Error)?;
        } else {
            self.d7.set_low().map_err(|_| Error)?;
        }

        // Pulse the enable pin
        self.en.set_high().map_err(|_| Error)?;
        delay.delay_ms(2u32);
        self.en.set_low().map_err(|_| Error)?;

        if data {
            self.rs.set_low().map_err(|_| Error)?;
        }
        Ok(())
    }
}
