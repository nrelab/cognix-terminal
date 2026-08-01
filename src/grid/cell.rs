//! Cell representation for terminal grid
//! Adapted from Alacritty terminal under Apache license

use std::boxed::Box;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use super::row::Row;
use crate::ansi::control_sequence_parameters::{Color, NamedColor};

/// Default character for empty cells
pub const DEFAULT_CHAR: char = '\0';
pub const DEFAULT_CHAR_BYTE: u8 = b'\0';
pub const DEFAULT_CHAR_STR: &str = "\0";

/// Maximum byte length of a single cell's accumulated grapheme cluster
pub const MAX_GRAPHEME_BYTES: usize = 256;

/// Soft threshold for warning about unusually large grapheme clusters
const WARN_GRAPHEME_BYTES: usize = 128;

bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct Flags: u16 {
        const INVERSE                   = 0b0000_0000_0000_0001;
        const BOLD                      = 0b0000_0000_0000_0010;
        const ITALIC                    = 0b0000_0000_0000_0100;
        const BOLD_ITALIC               = 0b0000_0000_0000_0110;
        const UNDERLINE                 = 0b0000_0000_0000_1000;
        const WRAPLINE                  = 0b0000_0000_0001_0000;
        const WIDE_CHAR                 = 0b0000_0000_0010_0000;
        const WIDE_CHAR_SPACER          = 0b0000_0000_0100_0000;
        const DIM                       = 0b0000_0000_1000_0000;
        const DIM_BOLD                  = 0b0000_0000_1000_0010;
        const HIDDEN                    = 0b0000_0001_0000_0000;
        const STRIKEOUT                 = 0b0000_0010_0000_0000;
        const LEADING_WIDE_CHAR_SPACER  = 0b0000_0100_0000_0000;
        const DOUBLE_UNDERLINE          = 0b0000_1000_0000_0000;
        const HAS_CURSOR                = 0b0001_0000_0000_0000;
        const CELL_DECORATIONS          = 0b0000_1010_0000_1000;
    }
}

// Use legacy serialization for bitflags compatibility
impl serde::Serialize for Flags {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        bitflags_serde_legacy::serialize(self, "Flags", serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Flags {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        bitflags_serde_legacy::deserialize("Flags", deserializer)
    }
}

/// Trait for determining if a reset should be performed
pub trait ResetDiscriminant<T> {
    fn discriminant(&self) -> T;
}

impl<T: Copy> ResetDiscriminant<T> for T {
    fn discriminant(&self) -> T {
        *self
    }
}

impl ResetDiscriminant<Color> for Cell {
    fn discriminant(&self) -> Color {
        self.bg
    }
}

/// Marker for end of prompt content
#[derive(Serialize, Deserialize, Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct EndOfPromptMarker {
    pub has_extra_trailing_newline: bool,
}

/// Dynamically allocated cell content for rarely-used attributes
#[derive(Serialize, Deserialize, Default, Debug, Clone, Eq, PartialEq)]
struct CellExtra {
    cell_with_zero_width: Option<String>,
    end_of_prompt: Option<EndOfPromptMarker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hyperlink_id: Option<u32>,
}

/// Content and attributes of a single cell in the terminal grid
/// Memory-optimized: 24 bytes total (4 + 5 + 5 + 2 + 8)
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Cell {
    pub c: char,
    pub fg: Color,
    pub bg: Color,
    pub flags: Flags,
    extra: Option<Box<CellExtra>>,
}

impl Default for Cell {
    #[inline]
    fn default() -> Cell {
        Cell {
            c: DEFAULT_CHAR,
            bg: Color::Named(NamedColor::Background),
            fg: Color::Named(NamedColor::Foreground),
            flags: Flags::empty(),
            extra: None,
        }
    }
}

impl Cell {
    /// Cell's character followed by all zerowidth characters
    #[inline]
    fn content_with_zerowidth(&self) -> Option<&str> {
        self.extra
            .as_ref()
            .and_then(|extra| extra.cell_with_zero_width.as_deref())
    }

    /// Returns the content for display purposes
    #[inline]
    pub fn content_for_display(&self) -> String {
        match self.content_with_zerowidth() {
            Some(content) => content.to_string(),
            None if self.c == DEFAULT_CHAR => " ".to_string(),
            None => {
                // Single character - return it as a string
                self.c.to_string()
            }
        }
    }

    /// Returns the raw cell content (may include non-printable markers)
    pub fn raw_content(&self) -> char {
        self.c
    }

    /// Write a new zerowidth character to this cell
    #[inline]
    pub fn push_zerowidth(&mut self, c: char, log_long_grapheme_warnings: bool) {
        if self.c == DEFAULT_CHAR {
            self.c = ' ';
        }

        let extra = self.extra.get_or_insert_with(Box::default);
        match &mut extra.cell_with_zero_width {
            Some(zerowidth) => {
                let old_len = zerowidth.len();
                let new_len = old_len + c.len_utf8();
                if new_len > MAX_GRAPHEME_BYTES {
                    return;
                }
                zerowidth.push(c);
                if log_long_grapheme_warnings
                    && old_len < WARN_GRAPHEME_BYTES
                    && new_len >= WARN_GRAPHEME_BYTES
                {
                    log::warn!(
                        "cell grapheme has accumulated {new_len} bytes of zero-width content"
                    );
                }
            }
            None => {
                extra.cell_with_zero_width = Some(format!("{}{}", self.c, c));
            }
        }
    }

    /// Returns whether cell is the end of prompt content
    #[inline]
    pub fn is_end_of_prompt(&self) -> bool {
        self.end_of_prompt_marker().is_some()
    }

    /// Returns end-of-prompt marker if present
    pub fn end_of_prompt_marker(&self) -> Option<EndOfPromptMarker> {
        self.extra.as_ref()?.end_of_prompt
    }

    /// Mark cell as the end of prompt content
    #[inline]
    pub fn mark_end_of_prompt(&mut self, has_extra_trailing_newline: bool) {
        self.extra
            .get_or_insert_with(Default::default)
            .end_of_prompt = Some(EndOfPromptMarker {
            has_extra_trailing_newline,
        });
    }

    /// Returns this cell's hyperlink id, if any
    #[inline]
    pub fn hyperlink_id(&self) -> Option<u32> {
        self.extra.as_ref()?.hyperlink_id
    }

    #[inline]
    pub fn set_hyperlink_id(&mut self, id: Option<u32>) {
        if id.is_some() {
            self.extra.get_or_insert_with(Default::default).hyperlink_id = id;
        } else if let Some(extra) = self.extra.as_deref_mut() {
            extra.hyperlink_id = None;
        }
    }

    /// Free all dynamically allocated cell storage
    #[inline]
    pub fn drop_extra(&mut self) {
        if let Some(extra) = self.extra.take()
            && let Some(end_of_prompt_marker) = extra.end_of_prompt
        {
            self.mark_end_of_prompt(end_of_prompt_marker.has_extra_trailing_newline);
        }
    }

    /// Check if cell is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.c == DEFAULT_CHAR
            && self.bg == Color::Named(NamedColor::Background)
            && self.fg == Color::Named(NamedColor::Foreground)
            && !self.flags.intersects(
                Flags::INVERSE
                    | Flags::UNDERLINE
                    | Flags::DOUBLE_UNDERLINE
                    | Flags::STRIKEOUT
                    | Flags::WRAPLINE
                    | Flags::WIDE_CHAR_SPACER
                    | Flags::LEADING_WIDE_CHAR_SPACER
                    | Flags::HAS_CURSOR,
            )
    }

    /// Returns whether rendering the cell would produce anything visible
    pub fn is_visible(&self) -> bool {
        !self.is_empty() && !self.c.is_ascii_whitespace()
    }

    #[inline]
    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    #[inline]
    pub fn flags_mut(&mut self) -> &mut Flags {
        &mut self.flags
    }

    #[inline]
    pub fn reset(&mut self, template: &Self) {
        *self = Cell {
            bg: template.bg,
            ..Cell::default()
        };
    }
}

impl From<Color> for Cell {
    #[inline]
    fn from(color: Color) -> Self {
        Self {
            bg: color,
            ..Cell::default()
        }
    }
}

/// Get the length of occupied cells in a line
pub trait LineLength {
    fn line_length(&self) -> usize;
}

impl LineLength for Row {
    fn line_length(&self) -> usize {
        if self.len() == 0 {
            return 0;
        }
        let mut length = 0;

        if self[self.len() - 1].flags.contains(Flags::WRAPLINE) {
            return self.len();
        }

        for (index, cell) in self[..].iter().rev().enumerate() {
            if cell.c != DEFAULT_CHAR {
                length = self.len() - index;
                break;
            }
        }

        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_default() {
        let cell = Cell::default();
        assert_eq!(cell.c, DEFAULT_CHAR);
        assert!(cell.is_empty());
    }

    #[test]
    fn test_cell_reset() {
        let mut cell = Cell::default();
        cell.c = 'a';
        cell.flags |= Flags::BOLD;
        
        let template = Cell::from(Color::Named(NamedColor::Red));
        cell.reset(&template);
        
        assert_eq!(cell.c, DEFAULT_CHAR);
        assert_eq!(cell.bg, Color::Named(NamedColor::Red));
    }

    #[test]
    fn test_cell_mark_end_of_prompt() {
        let mut cell = Cell::default();
        cell.mark_end_of_prompt(true);
        assert!(cell.is_end_of_prompt());
    }
}
