//! cognix-terminal - Terminal emulation library
//!
//! This library provides terminal emulation capabilities extracted from Warp,
//! including VTE/ANSI parsing, grid state management, and terminal mode handling.

pub mod ansi;
pub mod grid;
pub mod indexing;
pub mod mode;
pub mod pty;
pub mod render;
pub mod terminal;

pub use ansi::{control_sequence_parameters, escape_sequences};
pub use grid::{cell, row};
pub use indexing::{Point, VisiblePoint, VisibleRow};
pub use mode::{KeyboardModes, TermMode};
pub use pty::Pty;
pub use terminal::{Cursor, Size, Terminal};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::ansi::control_sequence_parameters::{
        Attr, Color, CursorShape, CursorStyle, NamedColor,
    };
    pub use crate::grid::cell::{Cell, Flags};
    pub use crate::grid::row::Row;
    pub use crate::indexing::Point;
    pub use crate::mode::TermMode;
    pub use crate::pty::Pty;
    pub use crate::terminal::Terminal;
}
