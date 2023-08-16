bitflags! {
    struct EntryModeFlags: u8 {
        const ENTRY_MODE            = 0b0000_0100;
        const CURSOR_MOVE_INCREMENT = 0b0000_0010;
        const DISPLAY_SHIFT_ON      = 0b0000_0001;
        const CURSOR_MOVE_DECREMENT = 0b0000_0000;
        const DISPLAY_SHIFT_OFF     = 0b0000_0000;
    }
}

/// Enumeration of possible methods to move.
#[derive(Clone, Copy)]
pub enum CursorMode {
    /// Moves right.
    Increment,
    /// Moves left.
    Decrement,
}

impl From<CursorMode> for EntryModeFlags {
    fn from(direction: CursorMode) -> Self {
        match direction {
            CursorMode::Increment => EntryModeFlags::CURSOR_MOVE_INCREMENT,
            CursorMode::Decrement => EntryModeFlags::CURSOR_MOVE_DECREMENT,
        }
    }
}

/// Enumeration to set display shift.
#[derive(Clone, Copy)]
pub enum ShiftMode {
    /// Shifts display.
    On,
    /// Does not shift display.
    Off,
}

impl From<ShiftMode> for EntryModeFlags {
    fn from(shift: ShiftMode) -> Self {
        match shift {
            ShiftMode::On => EntryModeFlags::DISPLAY_SHIFT_ON,
            ShiftMode::Off => EntryModeFlags::DISPLAY_SHIFT_OFF,
        }
    }
}

/// A struct for creating display entry mode settings.
pub struct EntryMode {
    /// The direction to move the cursor.
    pub move_direction: CursorMode,
    /// Whether to shift the display.
    pub display_shift: ShiftMode,
}

impl EntryMode {
    /// Sets the direction the read/write cursor is moved when a character code is written to or
    /// read from the display.
    // pub fn set_move_direction(&mut self, direction: CursorMode) -> &mut Self {
    //     self.move_direction = direction;
    //     self
    // }

    /// Sets the display shift, which will be performed on character write, either `On` or `Off`.
    ///
    /// If display shift is enabled, it will seem as if the cursor does not move but the display
    /// does.
    ///
    /// **Note:** The display does not shift when reading.
    // pub fn set_display_shift(&mut self, shift: ShiftMode) -> &mut Self {
    //     self.display_shift = shift;
    //     self
    // }

    pub fn as_byte(&self) -> u8 {
        let mut cmd = EntryModeFlags::ENTRY_MODE;

        cmd |= EntryModeFlags::from(self.move_direction);
        cmd |= EntryModeFlags::from(self.display_shift);

        cmd.bits()
    }
}

impl Default for EntryMode {
    /// Make a new `EntryMode` with the default settings described below.
    ///
    /// The default settings are:
    ///
    ///  - **move direction:**
    ///     - `Increment`
    ///  - **display_shift:**
    ///     - `Off`
    fn default() -> Self {
        Self {
            move_direction: CursorMode::Increment,
            display_shift: ShiftMode::Off,
        }
    }
}
