// #![deny(missing_docs)]
// #![deny(warnings)]
// #![deny(unsafe_code)]

/*!
Driver for I2C 16x2 character displays
Provides a driver for common 16x2 LCD displays that use the I2C bus to communicate.
This has been tested with the [LCD1602A module](https://robocraze.com/products/16x2-lcd-blue-with-i2c-interface).
*/

#![no_std]

#[macro_use]
extern crate bitflags;

/// The types of bus that can be used to communicate with the display.
pub mod data_bus;
use data_bus::{DataBus, EightBitBus, FourBitBus, I2CBus};

/// Display module for 16x2 LCD displays
pub mod display_control;
pub use display_control::{Cursor, CursorBlink, Display, DisplayMode};

/// Entry mode for 16x2 LCD displays
pub mod entry_mode;
use entry_mode::{CursorMode, EntryMode, ShiftMode};

/// Error types
pub mod error;
use error::Result;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::blocking::i2c;
use embedded_hal::digital::v2::OutputPin;

/**
Handles all the logic related to working with the character LCD via I2C. You'll
need to create an instance of this with the `new()` method.
The `I` generic type needs to implement the `embedded_hal::blocking::Write` trait.
*/
pub struct LCD1602<B: DataBus> {
    bus: B,
    entry_mode: EntryMode,
    display_mode: DisplayMode,
}

/// Used in the direction argument for shifting the cursor and the display
pub enum Direction {
    /// Shift left
    Left,
    /// Shift right
    Right,
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
    > LCD1602<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>>
{
    /// Create an instance of a `LCD1602` from 8 data pins, a register select
    /// pin, an enable pin and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to
    /// ensure the `LCD1602` has enough time to process commands.
    /// - The eight db0..db7 pins are used to send and recieve with
    ///  the `LCD1602`.
    /// - The register select pin is used to tell the `LCD1602`
    /// if incoming data is a command or data.
    /// - The enable pin is used to tell the `LCD1602` that there
    /// is data on the 8 data pins and that it should read them in.
    ///
    pub fn new_8bit<D: DelayUs<u16> + DelayMs<u8>>(
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
        delay: &mut D,
    ) -> Result<LCD1602<EightBitBus<RS, EN, D0, D1, D2, D3, D4, D5, D6, D7>>> {
        let mut hd = LCD1602 {
            bus: EightBitBus::from_pins(rs, en, d0, d1, d2, d3, d4, d5, d6, d7),
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_8bit(delay)?;

        return Ok(hd);
    }
}

impl<RS: OutputPin, EN: OutputPin, D4: OutputPin, D5: OutputPin, D6: OutputPin, D7: OutputPin>
    LCD1602<FourBitBus<RS, EN, D4, D5, D6, D7>>
{
    /// Create an instance of a `LCD1602` from 4 data pins, a register select
    /// pin, an enable pin and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to
    /// ensure the `LCD1602` has enough time to process commands.
    /// - The four db0..db3 pins are used to send and recieve with
    ///  the `LCD1602`.
    /// - The register select pin is used to tell the `LCD1602`
    /// if incoming data is a command or data.
    /// - The enable pin is used to tell the `LCD1602` that there
    /// is data on the 4 data pins and that it should read them in.
    ///
    /// This mode operates differently than 8 bit mode by using 4 less
    /// pins for data, which is nice on devices with less I/O although
    /// the I/O takes a 'bit' longer
    ///
    /// Instead of commands being sent byte by byte each command is
    /// broken up into it's upper and lower nibbles (4 bits) before
    /// being sent over the data bus
    ///
    pub fn new_4bit<D: DelayUs<u16> + DelayMs<u8>>(
        rs: RS,
        en: EN,
        d4: D4,
        d5: D5,
        d6: D6,
        d7: D7,
        delay: &mut D,
    ) -> Result<LCD1602<FourBitBus<RS, EN, D4, D5, D6, D7>>> {
        let mut hd = LCD1602 {
            bus: FourBitBus::from_pins(rs, en, d4, d5, d6, d7),
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_4bit(delay)?;

        return Ok(hd);
    }
}

impl<I2C: i2c::Write> LCD1602<I2CBus<I2C>> {
    /// Create an instance of a `LCD1602` from an i2c write peripheral,
    /// the `LCD1602` I2C address and a struct implementing the delay trait.
    /// - The delay instance is used to sleep between commands to
    /// ensure the `LCD1602` has enough time to process commands.
    /// - The i2c peripheral is used to send data to the `LCD1602` and to set
    /// its register select and enable pins.
    ///
    /// This mode operates on an I2C bus, using an I2C to parallel port expander
    ///
    pub fn new_i2c<D: DelayUs<u16> + DelayMs<u8>>(
        i2c_bus: I2C,
        address: u8,
        delay: &mut D,
    ) -> Result<LCD1602<I2CBus<I2C>>> {
        let mut hd = LCD1602 {
            bus: I2CBus::new(i2c_bus, address),
            entry_mode: EntryMode::default(),
            display_mode: DisplayMode::default(),
        };

        hd.init_4bit(delay)?;

        return Ok(hd);
    }
}

impl<B> LCD1602<B>
where
    B: DataBus,
{
    /// Unshifts the display and sets the cursor position to 0
    ///
    /// ```rust,ignore
    /// lcd.reset();
    /// ```
    pub fn reset<D: DelayUs<u16> + DelayMs<u8>>(&mut self, delay: &mut D) -> Result<()> {
        self.write_command(0b0000_0010, delay)?;

        Ok(())
    }

    /// Set if the display should be on, if the cursor should be
    /// visible, and if the cursor should blink
    ///
    /// Note: This is equivilent to calling all of the other relavent
    /// methods however this operation does it all in one go to the `LCD1602`
    pub fn set_display_mode<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        display_mode: DisplayMode,
        delay: &mut D,
    ) -> Result<()> {
        self.display_mode = display_mode;

        let cmd_byte = self.display_mode.as_byte();

        self.write_command(cmd_byte, delay)?;

        Ok(())
    }

    /// Clear the entire display
    ///
    /// ```rust,ignore
    /// lcd.clear();
    /// ```
    pub fn clear<D: DelayUs<u16> + DelayMs<u8>>(&mut self, delay: &mut D) -> Result<()> {
        self.write_command(0b0000_0001, delay)?;

        Ok(())
    }

    /// If enabled, automatically scroll the display when a new
    /// character is written to the display
    ///
    /// ```rust,ignore
    /// lcd.set_autoscroll(true);
    /// ```
    pub fn set_autoscroll<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        enabled: ShiftMode,
        delay: &mut D,
    ) -> Result<()> {
        self.entry_mode.display_shift = enabled.into();

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd, delay)?;

        Ok(())
    }

    /// Set if the cursor should be visible
    pub fn set_cursor_visibility<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        visibility: Cursor,
        delay: &mut D,
    ) -> Result<()> {
        self.display_mode.cursor_visibility = visibility;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay)?;

        Ok(())
    }

    /// Set if the characters on the display should be visible
    pub fn set_display<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        display: Display,
        delay: &mut D,
    ) -> Result<()> {
        self.display_mode.display = display;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay)?;

        Ok(())
    }

    /// Set if the cursor should blink
    pub fn set_cursor_blink<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        blink: CursorBlink,
        delay: &mut D,
    ) -> Result<()> {
        self.display_mode.cursor_blink = blink;

        let cmd = self.display_mode.as_byte();

        self.write_command(cmd, delay)?;

        Ok(())
    }

    /// Set which way the cursor will move when a new character is written
    ///
    /// ```rust,ignore
    /// // Move right (Default) when a new character is written
    /// lcd.set_cursor_mode(CursorMode::Right)
    ///
    /// // Move left when a new character is written
    /// lcd.set_cursor_mode(CursorMode::Left)
    /// ```
    pub fn set_cursor_mode<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        mode: CursorMode,
        delay: &mut D,
    ) -> Result<()> {
        self.entry_mode.move_direction = mode;

        let cmd = self.entry_mode.as_byte();

        self.write_command(cmd, delay)?;

        Ok(())
    }

    /// Set the cursor position
    ///
    /// ```rust,ignore
    /// // Move to line 2
    /// lcd.set_cursor_pos(40)
    /// ```
    pub fn set_cursor_pos<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        position: u8,
        delay: &mut D,
    ) -> Result<()> {
        let lower_7_bits = 0b0111_1111 & position;

        self.write_command(0b1000_0000 | lower_7_bits, delay)?;

        Ok(())
    }

    /// Shift just the cursor to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_cursor(Direction::Left);
    /// lcd.shift_cursor(Direction::Right);
    /// ```
    pub fn shift_cursor<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        dir: Direction,
        delay: &mut D,
    ) -> Result<()> {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_0000 | bits | bits, delay)?;

        Ok(())
    }

    /// Shift the entire display to the left or the right
    ///
    /// ```rust,ignore
    /// lcd.shift_display(Direction::Left);
    /// lcd.shift_display(Direction::Right);
    /// ```
    pub fn shift_display<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        dir: Direction,
        delay: &mut D,
    ) -> Result<()> {
        let bits = match dir {
            Direction::Left => 0b0000_0000,
            Direction::Right => 0b0000_0100,
        };

        self.write_command(0b0001_1000 | bits, delay)?;

        Ok(())
    }

    /// Write a single character to the `LCD1602`. This `char` just gets downcast to a `u8`
    /// internally, so make sure that whatever character you're printing fits inside that range, or
    /// you can just use [write_byte](#method.write_byte) to have the compiler check for you.
    /// See the documentation on that function for more details about compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_char('A', &mut delay)?; // prints 'A'
    /// ```
    pub fn write_char<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        data: char,
        delay: &mut D,
    ) -> Result<()> {
        self.write_byte(data as u8, delay)
    }

    fn write_command<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        cmd: u8,
        delay: &mut D,
    ) -> Result<()> {
        self.bus.write(cmd, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);
        Ok(())
    }

    fn init_4bit<D: DelayUs<u16> + DelayMs<u8>>(&mut self, delay: &mut D) -> Result<()> {
        // Wait for the LCD to wakeup if it was off
        delay.delay_ms(15u8);

        // Initialize Lcd in 4-bit mode
        self.bus.write(0x33, false, delay)?;

        // Wait for the command to be processed
        delay.delay_ms(5u8);

        // Sets 4-bit operation and enables 5x7 mode for chars
        self.bus.write(0x32, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        self.bus.write(0x28, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Clear Display
        self.bus.write(0x0E, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Move the cursor to beginning of first line
        self.bus.write(0x01, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Set entry mode
        self.bus.write(self.entry_mode.as_byte(), false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        self.bus.write(0x80, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        Ok(())
    }

    // Follow the 8-bit setup procedure as specified in the LCD1602 datasheet
    fn init_8bit<D: DelayUs<u16> + DelayMs<u8>>(&mut self, delay: &mut D) -> Result<()> {
        // Wait for the LCD to wakeup if it was off
        delay.delay_ms(15u8);

        // Initialize Lcd in 8-bit mode
        self.bus.write(0b0011_0000, false, delay)?;

        // Wait for the command to be processed
        delay.delay_ms(5u8);

        // Sets 8-bit operation and enables 5x7 mode for chars
        self.bus.write(0b0011_1000, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        self.bus.write(0b0000_1110, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Clear Display
        self.bus.write(0b0000_0001, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Move the cursor to beginning of first line
        self.bus.write(0b000_0111, false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        // Set entry mode
        self.bus.write(self.entry_mode.as_byte(), false, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        Ok(())
    }

    /// Writes a string to the LCD1602. Internally, this just prints the string byte-by-byte, so
    /// make sure the characters in the string fit in a normal `u8`. See the documentation on
    /// [write_byte](#method.write_byte) for more details on compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_str("Hello, World!", &mut delay)?;
    /// ```
    pub fn write_str<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        string: &str,
        delay: &mut D,
    ) -> Result<()> {
        self.write_bytes(string.as_bytes(), delay)
    }

    /// Writes a sequence of bytes to the LCD1602. See the documentation on the
    /// [write_byte](#method.write_byte) function for more details about compatibility.
    ///
    /// ```rust,ignore
    /// lcd.write_bytes(b"Hello, World!", &mut delay)?;
    /// ```
    pub fn write_bytes<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        string: &[u8],
        delay: &mut D,
    ) -> Result<()> {
        for &b in string {
            self.write_byte(b, delay)?;
        }
        Ok(())
    }

    /// Writes a single byte to the LCD1602. These usually map to ASCII characters when printed on the
    /// screen, but not always. While it varies depending on the ROM of the LCD, `0x20u8..=0x5b`
    /// and `0x5d..=0x7d` should map to their standard ASCII characters. That is, all the printable
    /// ASCII characters work, excluding `\` and `~`, which are usually displayed as `¥` and `🡢`
    /// respectively.
    ///
    /// More information can be found in the Hitachi datasheets for the LCD1602.
    ///
    /// ```rust,ignore
    /// lcd.write_byte(b'A', &mut delay)?; // prints 'A'
    /// lcd.write_byte(b'\\', &mut delay)?; // usually prints ¥
    /// lcd.write_byte(b'~', &mut delay)?; // usually prints 🡢
    /// lcd.write_byte(b'\x7f', &mut delay)?; // usually prints 🡠
    /// ```
    pub fn write_byte<D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        data: u8,
        delay: &mut D,
    ) -> Result<()> {
        self.bus.write(data, true, delay)?;

        // Wait for the command to be processed
        delay.delay_us(100);

        Ok(())
    }

    // Pulse the enable pin telling the LCD1602 that we something for it
    /*fn pulse_enable(&mut self) {
        self.en.set_high();
        self.delay.delay_ms(15u8);
        self.en.set_low();
    }*/
}

//impl<B> Write for LCD1602<B>
//where
//    B: DataBus,
//{
//    fn write_str(&mut self, string: &str) -> Result {
//        for c in string.chars() {
//            self.write_char(c, delay);
//        }
//        Ok(())
//    }
//}
