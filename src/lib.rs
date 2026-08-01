//! cognix-terminal - Terminal emulation library
//!
//! This library provides terminal emulation capabilities extracted from Warp,
//! including VTE/ANSI parsing, grid state management, and terminal mode handling.

pub mod ansi;
pub mod grid;
pub mod mode;
pub mod indexing;
pub mod terminal;
pub mod pty;
pub mod render;

pub use ansi::{control_sequence_parameters, escape_sequences};
pub use grid::{cell, row};
pub use mode::{TermMode, KeyboardModes};
pub use indexing::{Point, VisiblePoint, VisibleRow};
pub use terminal::{Terminal, Size, Cursor};
pub use pty::Pty;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::ansi::control_sequence_parameters::{
        Color, NamedColor, Attr, CursorStyle, CursorShape,
    };
    pub use crate::grid::cell::{Cell, Flags};
    pub use crate::grid::row::Row;
    pub use crate::mode::TermMode;
    pub use crate::indexing::Point;
    pub use crate::terminal::Terminal;
    pub use crate::pty::Pty;
}
