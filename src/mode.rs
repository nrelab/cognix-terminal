// Terminal mode management
// Adapted from Warp's terminal emulation layer

use bitflags::bitflags;

bitflags! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct TermMode: u32 {
        const NONE                = 0;
        const SHOW_CURSOR         = 0b0000_0000_0000_0000_0001;
        const APP_CURSOR          = 0b0000_0000_0000_0000_0010;
        const APP_KEYPAD          = 0b0000_0000_0000_0000_0100;
        const MOUSE_REPORT_CLICK  = 0b0000_0000_0000_0000_1000;
        const BRACKETED_PASTE     = 0b0000_0000_0000_0001_0000;
        const SGR_MOUSE           = 0b0000_0000_0000_0010_0000;
        const MOUSE_MOTION        = 0b0000_0000_0000_0100_0000;
        const LINE_WRAP           = 0b0000_0000_0000_1000_0000;
        const LINE_FEED_NEW_LINE  = 0b0000_0000_0001_0000_0000;
        const ORIGIN              = 0b0000_0000_0010_0000_0000;
        const INSERT              = 0b0000_0000_0100_0000_0000;
        const FOCUS_IN_OUT        = 0b0000_0000_1000_0000_0000;
        const MOUSE_DRAG          = 0b0000_0010_0000_0000_0000;
        const MOUSE_MODE          = 0b0000_0010_0000_0100_1000;
        const UTF8_MOUSE          = 0b0000_0100_0000_0000_0000;
        const ALTERNATE_SCROLL    = 0b0000_1000_0000_0000_0000;
        const VI                  = 0b0001_0000_0000_0000_0000;
        const URGENCY_HINTS       = 0b0010_0000_0000_0000_0000;

        // Kitty keyboard protocol enhancement flags
        const KEYBOARD_DISAMBIGUATE_ESCAPE     = 0b0100_0000_0000_0000_0000;
        const KEYBOARD_REPORT_EVENT_TYPES      = 0b1000_0000_0000_0000_0000;
        const KEYBOARD_REPORT_ALTERNATE_KEYS   = 0b0001_0000_0000_0000_0000_0000;
        const KEYBOARD_REPORT_ALL_AS_ESCAPE    = 0b0010_0000_0000_0000_0000_0000;
        const KEYBOARD_REPORT_ASSOCIATED_TEXT  = 0b0100_0000_0000_0000_0000_0000;

        const KEYBOARD_PROTOCOL = Self::KEYBOARD_DISAMBIGUATE_ESCAPE.bits()
            | Self::KEYBOARD_REPORT_EVENT_TYPES.bits()
            | Self::KEYBOARD_REPORT_ALTERNATE_KEYS.bits()
            | Self::KEYBOARD_REPORT_ALL_AS_ESCAPE.bits()
            | Self::KEYBOARD_REPORT_ASSOCIATED_TEXT.bits();

        const ANY                 = u32::MAX;
    }
}

impl Default for TermMode {
    fn default() -> TermMode {
        TermMode::SHOW_CURSOR
            | TermMode::LINE_WRAP
            | TermMode::ALTERNATE_SCROLL
            | TermMode::URGENCY_HINTS
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    pub struct KeyboardModes: u32 {
        const NO_MODE = 0;
        const DISAMBIGUATE_ESC_CODES = 0b0000_0001;
        const REPORT_EVENT_TYPES = 0b0000_0010;
        const REPORT_ALTERNATE_KEYS = 0b0000_0100;
        const REPORT_ALL_KEYS_AS_ESC = 0b0000_1000;
        const REPORT_ASSOCIATED_TEXT = 0b0001_0000;
    }
}

impl From<KeyboardModes> for TermMode {
    fn from(modes: KeyboardModes) -> Self {
        let mut term_mode = TermMode::NONE;

        if modes.contains(KeyboardModes::DISAMBIGUATE_ESC_CODES) {
            term_mode |= TermMode::KEYBOARD_DISAMBIGUATE_ESCAPE;
        }
        if modes.contains(KeyboardModes::REPORT_EVENT_TYPES) {
            term_mode |= TermMode::KEYBOARD_REPORT_EVENT_TYPES;
        }
        if modes.contains(KeyboardModes::REPORT_ALTERNATE_KEYS) {
            term_mode |= TermMode::KEYBOARD_REPORT_ALTERNATE_KEYS;
        }
        if modes.contains(KeyboardModes::REPORT_ALL_KEYS_AS_ESC) {
            term_mode |= TermMode::KEYBOARD_REPORT_ALL_AS_ESCAPE;
        }
        if modes.contains(KeyboardModes::REPORT_ASSOCIATED_TEXT) {
            term_mode |= TermMode::KEYBOARD_REPORT_ASSOCIATED_TEXT;
        }

        term_mode
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum KeyboardModesApplyBehavior {
    Replace,
    Union,
    Difference,
}

impl KeyboardModesApplyBehavior {
    pub fn from_kitty_apply_mode(mode: u16) -> Option<Self> {
        match mode {
            1 => Some(Self::Replace),
            2 => Some(Self::Union),
            3 => Some(Self::Difference),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_term_mode() {
        let mode = TermMode::default();
        assert!(mode.contains(TermMode::SHOW_CURSOR));
        assert!(mode.contains(TermMode::LINE_WRAP));
    }

    #[test]
    fn test_keyboard_modes_conversion() {
        let mut modes = KeyboardModes::DISAMBIGUATE_ESC_CODES;
        modes |= KeyboardModes::REPORT_EVENT_TYPES;
        
        let term_mode: TermMode = modes.into();
        assert!(term_mode.contains(TermMode::KEYBOARD_DISAMBIGUATE_ESCAPE));
        assert!(term_mode.contains(TermMode::KEYBOARD_REPORT_EVENT_TYPES));
    }
}
