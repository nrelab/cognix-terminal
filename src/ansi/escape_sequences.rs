//! Escape sequence definitions and encoding

/// C0 set of 7-bit control characters (from ANSI X3.4-1977)
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod C0 {
    pub const NUL: u8 = 0x00;
    pub const SOH: u8 = 0x01;
    pub const STX: u8 = 0x02;
    pub const ETX: u8 = 0x03;
    pub const EOT: u8 = 0x04;
    pub const ENQ: u8 = 0x05;
    pub const ACK: u8 = 0x06;
    pub const BEL: u8 = 0x07;
    pub const BS: u8 = 0x08;
    pub const HT: u8 = 0x09;
    pub const LF: u8 = 0x0A;
    pub const VT: u8 = 0x0B;
    pub const FF: u8 = 0x0C;
    pub const CR: u8 = 0x0D;
    pub const SO: u8 = 0x0E;
    pub const SI: u8 = 0x0F;
    pub const DLE: u8 = 0x10;
    pub const XON: u8 = 0x11;
    pub const DC2: u8 = 0x12;
    pub const XOFF: u8 = 0x13;
    pub const DC4: u8 = 0x14;
    pub const NAK: u8 = 0x15;
    pub const SYN: u8 = 0x16;
    pub const ETB: u8 = 0x17;
    pub const CAN: u8 = 0x18;
    pub const EM: u8 = 0x19;
    pub const SUB: u8 = 0x1A;
    pub const ESC: u8 = 0x1B;
    pub const FS: u8 = 0x1C;
    pub const GS: u8 = 0x1D;
    pub const RS: u8 = 0x1E;
    pub const US: u8 = 0x1F;
    pub const DEL: u8 = 0x7f;
}

/// C1 set of control characters (2-byte representations)
#[allow(non_snake_case)]
pub mod C1 {
    use super::C0::ESC;

    pub const IND: &[u8] = &[ESC, b'D'];
    pub const NEL: &[u8] = &[ESC, b'E'];
    pub const HTS: &[u8] = &[ESC, b'H'];
    pub const RI: &[u8] = &[ESC, b'M'];
    pub const SS2: &[u8] = &[ESC, b'N'];
    pub const SS3: &[u8] = &[ESC, b'O'];
    pub const DCS: &[u8] = &[ESC, b'P'];
    pub const SPA: &[u8] = &[ESC, b'V'];
    pub const EPA: &[u8] = &[ESC, b'W'];
    pub const SOS: &[u8] = &[ESC, b'X'];
    pub const DECID: &[u8] = &[ESC, b'Z'];
    pub const CSI: &[u8] = &[ESC, b'['];
    pub const ST: &[u8] = &[ESC, b'\\'];
    pub const OSC: &[u8] = &[ESC, b']'];
    pub const PM: &[u8] = &[ESC, b'^'];
    pub const APC: &[u8] = &[ESC, b'_'];

    pub fn to_utf8(c1_sequence: &[u8]) -> &str {
        std::str::from_utf8(c1_sequence).expect("Invalid C1 sequence")
    }
}

/// Bracketed paste mode sequences
pub const BRACKETED_PASTE_START: &[u8] = &[C0::ESC, b'[', b'2', b'0', b'0', b'~'];
pub const BRACKETED_PASTE_END: &[u8] = &[C0::ESC, b'[', b'2', b'0', b'1', b'~'];

#[allow(non_snake_case)]
pub mod EscCodes {
    use super::C0;

    pub const ARROW_UP: u8 = b'A';
    pub const ARROW_DOWN: u8 = b'B';
    pub const ARROW_RIGHT: u8 = b'C';
    pub const ARROW_LEFT: u8 = b'D';

    pub const WORD_LEFT: &[u8] = &[C0::ESC, b'b'];
    pub const WORD_RIGHT: &[u8] = &[C0::ESC, b'f'];

    pub const PAGE_UP: &[u8] = b"5~";
    pub const PAGE_DOWN: &[u8] = b"6~";
    pub const BACKWARD_TABULATION: &[u8] = b"Z";

    pub const HOME: u8 = b'H';
    pub const END: u8 = b'F';

    pub const MOUSE_LEFT: u8 = 0;
    pub const MOUSE_RIGHT: u8 = 2;
    pub const MOUSE_DRAG: u8 = 32;
    pub const MOUSE_MOVE: u8 = 35;
    pub const MOUSE_WHEEL_UP: u8 = 64;
    pub const MOUSE_WHEEL_DOWN: u8 = 65;

    pub const FOCUS_IN: &[u8] = &[C0::ESC, b'[', b'I'];
    pub const FOCUS_OUT: &[u8] = &[C0::ESC, b'[', b'O'];

    pub fn build_escape_sequence_with_c1(c1: &[u8], c: &[u8]) -> Vec<u8> {
        let mut sequence = Vec::new();
        sequence.extend_from_slice(c1);
        sequence.extend_from_slice(c);
        sequence
    }

    pub fn build_escape_sequence(c1_sequence: &[u8], byte: u8) -> Vec<u8> {
        let mut seq = c1_sequence.to_vec();
        seq.push(byte);
        seq
    }
}

/// Trait for objects that can provide information about the terminal's mode
pub trait ModeProvider {
    fn is_term_mode_set(&self, mode: crate::mode::TermMode) -> bool;
}

#[cfg(test)]
mod tests {
    use super::{C0, C1, EscCodes};

    #[test]
    fn test_c1_to_utf8() {
        assert_eq!(C1::to_utf8(C1::CSI), "\x1b[");
    }
    #[test]
    fn test_build_escape_sequence() {
        let seq = EscCodes::build_escape_sequence(C1::CSI, b'A');
        assert_eq!(seq, vec![0x1b, b'[', b'A']);
    }

    #[test]
    fn test_c0_codes() {
        assert_eq!(C0::NUL, 0x00);
        assert_eq!(C0::BEL, 0x07);
        assert_eq!(C0::ESC, 0x1B);
    }
}
