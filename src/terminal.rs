//! Terminal model coordinator
//! Minimal terminal emulation implementation

use crate::ansi::control_sequence_parameters::{Color, NamedColor};
use crate::grid::{Cell, Row};
use crate::indexing::Point;
use crate::mode::TermMode;

/// Terminal dimensions
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub rows: usize,
    pub cols: usize,
}

impl Size {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn area(&self) -> usize {
        self.rows * self.cols
    }
}

/// Terminal cursor state
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub point: Point,
    pub visible: bool,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            point: Point::zero(),
            visible: true,
        }
    }
}

/// Minimal terminal model
pub struct Terminal {
    /// Grid of cells (rows × cols)
    grid: Vec<Row>,
    /// Current cursor position
    cursor: Cursor,
    /// Terminal mode flags
    mode: TermMode,
    /// Terminal dimensions
    size: Size,
    /// Current foreground color
    fg_color: Color,
    /// Current background color
    bg_color: Color,
}

impl Terminal {
    /// Create a new terminal with specified dimensions
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = Size::new(rows, cols);
        let mut grid = Vec::with_capacity(rows);

        for _ in 0..rows {
            grid.push(Row::new(cols));
        }

        Self {
            grid,
            cursor: Cursor::default(),
            mode: TermMode::default(),
            size,
            fg_color: Color::Named(NamedColor::Foreground),
            bg_color: Color::Named(NamedColor::Background),
        }
    }

    /// Get terminal dimensions
    pub fn size(&self) -> Size {
        self.size
    }

    /// Resize the terminal
    pub fn resize(&mut self, rows: usize, cols: usize) {
        self.size = Size::new(rows, cols);

        // Adjust grid size
        if rows > self.grid.len() {
            for _ in self.grid.len()..rows {
                self.grid.push(Row::new(cols));
            }
        } else {
            self.grid.truncate(rows);
        }

        // Adjust each row's column count
        for row in &mut self.grid {
            row.grow(cols);
        }
    }

    /// Get the grid
    pub fn grid(&self) -> &[Row] {
        &self.grid
    }

    /// Get mutable grid
    pub fn grid_mut(&mut self) -> &mut [Row] {
        &mut self.grid
    }

    /// Get cursor position
    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Get terminal mode
    pub fn mode(&self) -> TermMode {
        self.mode
    }

    /// Set terminal mode
    pub fn set_mode(&mut self, mode: TermMode) {
        self.mode = mode;
    }

    /// Write a character to the current cursor position
    pub fn write_char(&mut self, c: char) {
        let row = self.cursor.point.row;
        let col = self.cursor.point.col;

        if row < self.grid.len() && col < self.grid[row].len() {
            let cell = &mut self.grid[row][col];
            cell.c = c;
            cell.fg = self.fg_color;
            cell.bg = self.bg_color;
        }

        // Advance cursor
        self.advance_cursor(1);
    }

    /// Write a string to the terminal
    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            match c {
                '\n' => self.linefeed(),
                '\r' => self.carriage_return(),
                '\t' => self.tab(),
                _ => self.write_char(c),
            }
        }
    }

    /// Process bytes from PTY
    pub fn input(&mut self, bytes: &[u8]) {
        // Simple implementation: treat as UTF-8
        if let Ok(s) = std::str::from_utf8(bytes) {
            self.write_str(s);
        }
    }

    /// Clear the entire screen
    pub fn clear(&mut self) {
        let template = Cell::from(self.bg_color);
        for row in &mut self.grid {
            row.reset(&template);
        }
        self.cursor.point = Point::zero();
    }

    /// Clear from cursor to end of line
    pub fn clear_line(&mut self) {
        let row = self.cursor.point.row;
        let col = self.cursor.point.col;

        if row < self.grid.len() {
            let template = Cell::from(self.bg_color);
            for cell in &mut self.grid[row][col..] {
                cell.reset(&template);
            }
        }
    }

    /// Move cursor to specified position
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor.point.row = row.min(self.size.rows - 1);
        self.cursor.point.col = col.min(self.size.cols - 1);
    }

    /// Move cursor forward by n positions
    pub fn cursor_forward(&mut self, n: usize) {
        self.cursor.point.col = (self.cursor.point.col + n).min(self.size.cols - 1);
    }

    /// Move cursor backward by n positions
    pub fn cursor_backward(&mut self, n: usize) {
        self.cursor.point.col = self.cursor.point.col.saturating_sub(n);
    }

    /// Move cursor down by n lines
    pub fn cursor_down(&mut self, n: usize) {
        self.cursor.point.row = (self.cursor.point.row + n).min(self.size.rows - 1);
    }

    /// Move cursor up by n lines
    pub fn cursor_up(&mut self, n: usize) {
        self.cursor.point.row = self.cursor.point.row.saturating_sub(n);
    }

    /// Set foreground color
    pub fn set_fg_color(&mut self, color: Color) {
        self.fg_color = color;
    }

    /// Set background color
    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color = color;
    }

    /// Reset colors to defaults
    pub fn reset_colors(&mut self) {
        self.fg_color = Color::Named(NamedColor::Foreground);
        self.bg_color = Color::Named(NamedColor::Background);
    }

    // Private helper methods

    fn advance_cursor(&mut self, n: usize) {
        self.cursor_forward(n);

        // Handle line wrapping
        if self.cursor.point.col >= self.size.cols {
            self.cursor.point.col = 0;
            self.linefeed();
        }
    }

    fn linefeed(&mut self) {
        self.cursor_down(1);
        self.carriage_return(); // Reset column to 0

        // Scroll if needed
        if self.cursor.point.row >= self.size.rows {
            self.scroll(1);
            self.cursor.point.row = self.size.rows - 1;
        }
    }

    fn carriage_return(&mut self) {
        self.cursor.point.col = 0;
    }

    fn tab(&mut self) {
        // Simple tab implementation: move to next multiple of 8
        let tab_stop = 8;
        let current = self.cursor.point.col;
        let next = ((current / tab_stop) + 1) * tab_stop;
        self.cursor_forward(next - current);
    }

    fn scroll(&mut self, n: usize) {
        // Remove first n rows and append empty rows at the end
        for _ in 0..n {
            if !self.grid.is_empty() {
                self.grid.remove(0);
                self.grid.push(Row::new(self.size.cols));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_new() {
        let term = Terminal::new(24, 80);
        assert_eq!(term.size().rows, 24);
        assert_eq!(term.size().cols, 80);
        assert_eq!(term.grid().len(), 24);
    }

    #[test]
    fn test_write_char() {
        let mut term = Terminal::new(10, 10);
        term.write_char('A');

        assert_eq!(term.grid()[0][0].c, 'A');
        assert_eq!(term.cursor().point.col, 1);
    }

    #[test]
    fn test_write_str() {
        let mut term = Terminal::new(10, 10);
        term.write_str("Hello");

        assert_eq!(term.grid()[0][0].c, 'H');
        assert_eq!(term.grid()[0][1].c, 'e');
        assert_eq!(term.cursor().point.col, 5);
    }

    #[test]
    fn test_linefeed() {
        let mut term = Terminal::new(10, 10);
        term.write_str("Hello\nWorld");

        assert_eq!(term.grid()[0][0].c, 'H');
        assert_eq!(term.grid()[1][0].c, 'W');
        assert_eq!(term.cursor().point.row, 1); // Cursor is on row 1 (where "World" is)
        // Linefeed moves cursor to next row and resets column
    }

    #[test]
    fn test_clear() {
        let mut term = Terminal::new(10, 10);
        term.write_str("Hello");
        term.clear();

        assert!(term.grid()[0].is_clear());
        assert_eq!(term.cursor().point, Point::zero());
    }

    #[test]
    fn test_resize() {
        let mut term = Terminal::new(10, 10);
        term.resize(20, 40);

        assert_eq!(term.size().rows, 20);
        assert_eq!(term.size().cols, 40);
        assert_eq!(term.grid().len(), 20);
    }
}
