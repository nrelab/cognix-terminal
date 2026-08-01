//! Control sequence parameters and terminal attributes
//! Adapted from vte crate (Alacritty project) under Apache license

use std::convert::TryFrom;
use std::{iter, str};

use serde::{Deserialize, Serialize};
use vte::ParamsIter;

/// Terminal cursor configuration
#[derive(Default, Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct CursorStyle {
    pub shape: CursorShape,
    pub blinking: bool,
}

/// Terminal cursor shape
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
pub enum CursorShape {
    #[default]
    Block,
    Underline,
    Beam,
    HollowBlock,
    Hidden,
}

/// Terminal modes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    CursorKeys,
    DECCOLM,
    Insert,
    Origin,
    LineWrap,
    BlinkingCursor,
    LineFeedNewLine,
    ShowCursor,
    ReportMouseClicks,
    ReportCellMouseMotion,
    ReportAllMouseMotion,
    ReportFocusInOut,
    Utf8Mouse,
    SgrMouse,
    AlternateScroll,
    UrgencyHints,
    SwapScreen { save_cursor_and_clear_screen: bool },
    BracketedPaste,
    SyncOutput,
}

impl Mode {
    pub fn from_primitive(intermediate: Option<&u8>, num: u16) -> Option<Mode> {
        if num == 0 {
            return None;
        }

        let private = match intermediate {
            Some(b'?') => true,
            None => false,
            _ => return None,
        };

        if private {
            Some(match num {
                1 => Mode::CursorKeys,
                3 => Mode::DECCOLM,
                6 => Mode::Origin,
                7 => Mode::LineWrap,
                12 => Mode::BlinkingCursor,
                25 => Mode::ShowCursor,
                1000 => Mode::ReportMouseClicks,
                1002 => Mode::ReportCellMouseMotion,
                1003 => Mode::ReportAllMouseMotion,
                1004 => Mode::ReportFocusInOut,
                1005 => Mode::Utf8Mouse,
                1006 => Mode::SgrMouse,
                1007 => Mode::AlternateScroll,
                1042 => Mode::UrgencyHints,
                47 => Mode::SwapScreen {
                    save_cursor_and_clear_screen: false,
                },
                1049 => Mode::SwapScreen {
                    save_cursor_and_clear_screen: true,
                },
                2004 => Mode::BracketedPaste,
                2026 => Mode::SyncOutput,
                _ => return None,
            })
        } else {
            Some(match num {
                4 => Mode::Insert,
                20 => Mode::LineFeedNewLine,
                _ => return None,
            })
        }
    }
}

/// Mode for clearing line
#[derive(Debug, Copy, Clone)]
pub enum LineClearMode {
    Right,
    Left,
    All,
}

/// Mode for clearing terminal
#[derive(Debug, Copy, Clone)]
pub enum ClearMode {
    Below,
    Above,
    All,
    Saved,
    ResetAndClear,
    ActiveBlock,
}

/// Mode for clearing tab stops
#[derive(Debug, Copy, Clone)]
pub enum TabulationClearMode {
    Current,
    All,
}

/// Standard colors
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Foreground,
    Background,
    Cursor,
    DimBlack,
    DimRed,
    DimGreen,
    DimYellow,
    DimBlue,
    DimMagenta,
    DimCyan,
    DimWhite,
    BrightForeground,
    DimForeground,
}

impl NamedColor {
    pub fn to_bright(self) -> Self {
        match self {
            NamedColor::Foreground => NamedColor::BrightForeground,
            NamedColor::Black => NamedColor::BrightBlack,
            NamedColor::Red => NamedColor::BrightRed,
            NamedColor::Green => NamedColor::BrightGreen,
            NamedColor::Yellow => NamedColor::BrightYellow,
            NamedColor::Blue => NamedColor::BrightBlue,
            NamedColor::Magenta => NamedColor::BrightMagenta,
            NamedColor::Cyan => NamedColor::BrightCyan,
            NamedColor::White => NamedColor::BrightWhite,
            NamedColor::DimForeground => NamedColor::Foreground,
            NamedColor::DimBlack => NamedColor::Black,
            NamedColor::DimRed => NamedColor::Red,
            NamedColor::DimGreen => NamedColor::Green,
            NamedColor::DimYellow => NamedColor::Yellow,
            NamedColor::DimBlue => NamedColor::Blue,
            NamedColor::DimMagenta => NamedColor::Magenta,
            NamedColor::DimCyan => NamedColor::Cyan,
            NamedColor::DimWhite => NamedColor::White,
            val => val,
        }
    }
}

/// Simple color representation (simplified from pathfinder_color)
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct ColorU {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorU {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
    Named(NamedColor),
    Spec(ColorU),
    Indexed(u8),
}

/// Terminal character attributes
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Attr {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    DoubleUnderline,
    BlinkSlow,
    BlinkFast,
    Reverse,
    Hidden,
    Strike,
    CancelBold,
    CancelBoldDim,
    CancelItalic,
    CancelUnderline,
    CancelBlink,
    CancelReverse,
    CancelHidden,
    CancelStrike,
    Foreground(Color),
    Background(Color),
}

/// Character set identifiers
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CharsetIndex {
    #[default]
    G0,
    G1,
    G2,
    G3,
}

/// Standard character sets
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StandardCharset {
    #[default]
    Ascii,
    SpecialCharacterAndLineDrawing,
}

impl StandardCharset {
    pub fn map(self, c: char) -> char {
        match self {
            StandardCharset::Ascii => c,
            StandardCharset::SpecialCharacterAndLineDrawing => match c {
                '`' => '◆',
                'a' => '▒',
                'b' => '\t',
                'c' => '\u{000c}',
                'd' => '\r',
                'e' => '\n',
                'f' => '°',
                'g' => '±',
                'h' => '\u{2424}',
                'i' => '\u{000b}',
                'j' => '┘',
                'k' => '┐',
                'l' => '┌',
                'm' => '└',
                'n' => '┼',
                'o' => '⎺',
                'p' => '⎻',
                'q' => '─',
                'r' => '⎼',
                's' => '⎽',
                't' => '├',
                'u' => '┤',
                'v' => '┴',
                'w' => '┬',
                'x' => '│',
                'y' => '≤',
                'z' => '≥',
                '{' => 'π',
                '|' => '≠',
                '}' => '£',
                '~' => '·',
                _ => c,
            },
        }
    }
}

pub fn attrs_from_sgr_parameters(params: &mut ParamsIter<'_>) -> Vec<Option<Attr>> {
    let mut attrs = Vec::with_capacity(params.size_hint().0);

    while let Some(param) = params.next() {
        let attr = match param {
            [0] => Some(Attr::Reset),
            [1] => Some(Attr::Bold),
            [2] => Some(Attr::Dim),
            [3] => Some(Attr::Italic),
            [4, 0] => Some(Attr::CancelUnderline),
            [4, 2] => Some(Attr::DoubleUnderline),
            [4, ..] => Some(Attr::Underline),
            [5] => Some(Attr::BlinkSlow),
            [6] => Some(Attr::BlinkFast),
            [7] => Some(Attr::Reverse),
            [8] => Some(Attr::Hidden),
            [9] => Some(Attr::Strike),
            [21] => Some(Attr::CancelBold),
            [22] => Some(Attr::CancelBoldDim),
            [23] => Some(Attr::CancelItalic),
            [24] => Some(Attr::CancelUnderline),
            [25] => Some(Attr::CancelBlink),
            [27] => Some(Attr::CancelReverse),
            [28] => Some(Attr::CancelHidden),
            [29] => Some(Attr::CancelStrike),
            [30] => Some(Attr::Foreground(Color::Named(NamedColor::Black))),
            [31] => Some(Attr::Foreground(Color::Named(NamedColor::Red))),
            [32] => Some(Attr::Foreground(Color::Named(NamedColor::Green))),
            [33] => Some(Attr::Foreground(Color::Named(NamedColor::Yellow))),
            [34] => Some(Attr::Foreground(Color::Named(NamedColor::Blue))),
            [35] => Some(Attr::Foreground(Color::Named(NamedColor::Magenta))),
            [36] => Some(Attr::Foreground(Color::Named(NamedColor::Cyan))),
            [37] => Some(Attr::Foreground(Color::Named(NamedColor::White))),
            [38] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(Attr::Foreground)
            }
            [38, params @ ..] => {
                let rgb_start = if params.len() > 4 { 2 } else { 1 };
                let rgb_iter = params[rgb_start..].iter().copied();
                let mut iter = iter::once(params[0]).chain(rgb_iter);
                parse_sgr_color(&mut iter).map(Attr::Foreground)
            }
            [39] => Some(Attr::Foreground(Color::Named(NamedColor::Foreground))),
            [40] => Some(Attr::Background(Color::Named(NamedColor::Black))),
            [41] => Some(Attr::Background(Color::Named(NamedColor::Red))),
            [42] => Some(Attr::Background(Color::Named(NamedColor::Green))),
            [43] => Some(Attr::Background(Color::Named(NamedColor::Yellow))),
            [44] => Some(Attr::Background(Color::Named(NamedColor::Blue))),
            [45] => Some(Attr::Background(Color::Named(NamedColor::Magenta))),
            [46] => Some(Attr::Background(Color::Named(NamedColor::Cyan))),
            [47] => Some(Attr::Background(Color::Named(NamedColor::White))),
            [48] => {
                let mut iter = params.map(|param| param[0]);
                parse_sgr_color(&mut iter).map(Attr::Background)
            }
            [48, params @ ..] => {
                let rgb_start = if params.len() > 4 { 2 } else { 1 };
                let rgb_iter = params[rgb_start..].iter().copied();
                let mut iter = iter::once(params[0]).chain(rgb_iter);
                parse_sgr_color(&mut iter).map(Attr::Background)
            }
            [49] => Some(Attr::Background(Color::Named(NamedColor::Background))),
            [90] => Some(Attr::Foreground(Color::Named(NamedColor::BrightBlack))),
            [91] => Some(Attr::Foreground(Color::Named(NamedColor::BrightRed))),
            [92] => Some(Attr::Foreground(Color::Named(NamedColor::BrightGreen))),
            [93] => Some(Attr::Foreground(Color::Named(NamedColor::BrightYellow))),
            [94] => Some(Attr::Foreground(Color::Named(NamedColor::BrightBlue))),
            [95] => Some(Attr::Foreground(Color::Named(NamedColor::BrightMagenta))),
            [96] => Some(Attr::Foreground(Color::Named(NamedColor::BrightCyan))),
            [97] => Some(Attr::Foreground(Color::Named(NamedColor::BrightWhite))),
            [100] => Some(Attr::Background(Color::Named(NamedColor::BrightBlack))),
            [101] => Some(Attr::Background(Color::Named(NamedColor::BrightRed))),
            [102] => Some(Attr::Background(Color::Named(NamedColor::BrightGreen))),
            [103] => Some(Attr::Background(Color::Named(NamedColor::BrightYellow))),
            [104] => Some(Attr::Background(Color::Named(NamedColor::BrightBlue))),
            [105] => Some(Attr::Background(Color::Named(NamedColor::BrightMagenta))),
            [106] => Some(Attr::Background(Color::Named(NamedColor::BrightCyan))),
            [107] => Some(Attr::Background(Color::Named(NamedColor::BrightWhite))),
            _ => None,
        };
        attrs.push(attr);
    }

    attrs
}

fn parse_sgr_color(params: &mut dyn Iterator<Item = u16>) -> Option<Color> {
    match params.next() {
        Some(2) => Some(Color::Spec(ColorU::new(
            u8::try_from(params.next()?).ok()?,
            u8::try_from(params.next()?).ok()?,
            u8::try_from(params.next()?).ok()?,
            0xff,
        ))),
        Some(5) => Some(Color::Indexed(u8::try_from(params.next()?).ok()?)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_style_default() {
        let style = CursorStyle::default();
        assert_eq!(style.shape, CursorShape::Block);
        assert!(!style.blinking);
    }

    #[test]
    fn test_named_color_to_bright() {
        assert_eq!(NamedColor::Red.to_bright(), NamedColor::BrightRed);
        assert_eq!(NamedColor::Black.to_bright(), NamedColor::BrightBlack);
    }

    #[test]
    fn test_mode_from_primitive() {
        assert_eq!(Mode::from_primitive(Some(&b'?'), 1), Some(Mode::CursorKeys));
        assert_eq!(Mode::from_primitive(None, 4), Some(Mode::Insert));
    }
}
