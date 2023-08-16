bitflags! {
    struct DisplayControlFlags: u8 {
        const DISPLAY_CONTROL       = 0b0000_1000;
        const DISPLAY_ON            = 0b0000_0100;
        const CURSOR_ON             = 0b0000_0010;
        const CURSOR_BLINKING_ON    = 0b0000_0001;
        const DISPLAY_OFF           = 0b0000_0000;
        const CURSOR_OFF            = 0b0000_0000;
        const CURSOR_BLINKING_OFF   = 0b0000_0000;
    }
}

/// State of a display.
#[derive(Clone, Copy)]
pub enum Display {
    /// Display is on.
    On,
    /// Display is off.
    Off,
}

impl From<Display> for DisplayControlFlags {
    fn from(state: Display) -> Self {
        match state {
            Display::On => DisplayControlFlags::DISPLAY_ON,
            Display::Off => DisplayControlFlags::DISPLAY_OFF,
        }
    }
}

/// State of a cursor.
#[derive(Clone, Copy)]
pub enum Cursor {
    /// Cursor is on.
    On,
    /// Cursor is off.
    Off,
}

impl From<Cursor> for DisplayControlFlags {
    fn from(state: Cursor) -> Self {
        match state {
            Cursor::On => DisplayControlFlags::CURSOR_ON,
            Cursor::Off => DisplayControlFlags::CURSOR_OFF,
        }
    }
}

/// Sets cursor blinking.
#[derive(Clone, Copy)]
pub enum CursorBlink {
    /// Cursor is blinking.
    On,
    /// Cursor is not blinking.
    Off,
}

impl From<CursorBlink> for DisplayControlFlags {
    fn from(state: CursorBlink) -> Self {
        match state {
            CursorBlink::On => DisplayControlFlags::CURSOR_BLINKING_ON,
            CursorBlink::Off => DisplayControlFlags::CURSOR_BLINKING_OFF,
        }
    }
}

/// A struct for creating display control settings.
pub struct DisplayMode {
    /// Whether to display the display.
    pub cursor_visibility: Cursor,
    /// Whether to display the cursor.
    pub cursor_blink: CursorBlink,
    /// Whether to blink the cursor.
    pub display: Display,
}

impl DisplayMode {
    /// Sets the entire display `On` or `Off`.
    ///
    /// Default is `On`.
    pub fn set_display(&mut self, state: Display) -> &mut Self {
        self.display = state;
        self
    }

    /// Sets the cursor `On` or `Off`.
    ///
    /// Default is `Off`.
    ///
    /// **Note:** This will not change cursor move direction or any other settings.
    pub fn set_cursor(&mut self, state: Cursor) -> &mut Self {
        self.cursor_visibility = state;
        self
    }

    /// Sets the blinking of the cursor `On` of `Off`.
    ///
    /// Default is `Off`.
    pub fn set_cursor_blinking(&mut self, blinking: CursorBlink) -> &mut Self {
        self.cursor_blink = blinking;
        self
    }

    /// Returns the display control flags.
    pub fn as_byte(&self) -> u8 {
        let mut cmd = DisplayControlFlags::DISPLAY_CONTROL;

        cmd |= DisplayControlFlags::from(self.display);
        cmd |= DisplayControlFlags::from(self.cursor_visibility);
        cmd |= DisplayControlFlags::from(self.cursor_blink);

        cmd.bits()
    }
}

impl Default for DisplayMode {
    /// Makes a new `DisplayControlBuilder` using the default settings described below.
    ///
    /// The default settings are:
    ///
    ///  - **display:**
    ///     - `On`
    ///  - **cursor:**
    ///     - `Off`
    ///  - **blinkinging of cursor:**
    ///     - `Off`
    fn default() -> Self {
        Self {
            cursor_visibility: Cursor::On,
            cursor_blink: CursorBlink::On,
            display: Display::On,
        }
    }
}
