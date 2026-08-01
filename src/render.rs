//! Basic rendering interface for terminal output
//! Provides text-based rendering of the terminal grid

use crate::ansi::control_sequence_parameters::{Color, NamedColor};
use crate::grid::Row;

/// Renderer configuration
#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    /// Show cursor position
    pub show_cursor: bool,
    /// Use ANSI colors in output
    pub use_colors: bool,
    /// Show line numbers
    pub show_line_numbers: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            show_cursor: true,
            use_colors: false,
            show_line_numbers: false,
        }
    }
}

/// Basic text renderer for terminal grid
pub struct Renderer {
    config: RenderConfig,
}

impl Renderer {
    /// Create a new renderer with default configuration
    pub fn new() -> Self {
        Self {
            config: RenderConfig::default(),
        }
    }

    /// Create a new renderer with custom configuration
    pub fn with_config(config: RenderConfig) -> Self {
        Self { config }
    }

    /// Render the terminal grid to a string
    pub fn render(&self, grid: &[Row], cursor_row: usize, cursor_col: usize) -> String {
        let mut output = String::new();

        for (row_idx, row) in grid.iter().enumerate() {
            if self.config.show_line_numbers {
                output.push_str(&format!("{:3} | ", row_idx));
            }

            for (col_idx, cell) in row.iter().enumerate() {
                let is_cursor = row_idx == cursor_row && col_idx == cursor_col;
                let char = if cell.c == '\0' { ' ' } else { cell.c };

                if self.config.show_cursor && is_cursor {
                    output.push('█');
                } else {
                    output.push(char);
                }
            }

            output.push('\n');
        }

        output
    }

    /// Render with ANSI color codes
    pub fn render_colored(&self, grid: &[Row], cursor_row: usize, cursor_col: usize) -> String {
        let mut output = String::new();

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let is_cursor = row_idx == cursor_row && col_idx == cursor_col;
                let char = if cell.c == '\0' { ' ' } else { cell.c };

                // Add color codes if enabled
                if self.config.use_colors {
                    output.push_str(&self.color_to_ansi(cell.fg, true));
                    output.push_str(&self.color_to_ansi(cell.bg, false));
                }

                if self.config.show_cursor && is_cursor {
                    output.push_str("\x1b[7m"); // Reverse video for cursor
                    output.push('█');
                    output.push_str("\x1b[27m"); // Reset reverse
                } else {
                    output.push(char);
                }

                if self.config.use_colors {
                    output.push_str("\x1b[0m"); // Reset colors
                }
            }

            output.push('\n');
        }

        output
    }

    /// Convert a color to ANSI escape code
    fn color_to_ansi(&self, color: Color, is_fg: bool) -> String {
        let (base_code, named_color) = match color {
            Color::Named(named) => (if is_fg { 30 } else { 40 }, named),
            Color::Spec(rgb) => {
                let r = rgb.r;
                let g = rgb.g;
                let b = rgb.b;
                let base = if is_fg { 38 } else { 48 };
                return format!("\x1b[{};2;{};{};{}m", base, r, g, b);
            }
            Color::Indexed(idx) => {
                let base = if is_fg { 38 } else { 48 };
                return format!("\x1b[{};5;{}m", base, idx);
            }
        };

        let color_code = match named_color {
            NamedColor::Black => 0,
            NamedColor::Red => 1,
            NamedColor::Green => 2,
            NamedColor::Yellow => 3,
            NamedColor::Blue => 4,
            NamedColor::Magenta => 5,
            NamedColor::Cyan => 6,
            NamedColor::White => 7,
            NamedColor::BrightBlack => 8,
            NamedColor::BrightRed => 9,
            NamedColor::BrightGreen => 10,
            NamedColor::BrightYellow => 11,
            NamedColor::BrightBlue => 12,
            NamedColor::BrightMagenta => 13,
            NamedColor::BrightCyan => 14,
            NamedColor::BrightWhite => 15,
            NamedColor::Foreground => 39,
            NamedColor::Background => 49,
            NamedColor::Cursor => 39,
            NamedColor::DimBlack => 0,
            NamedColor::DimRed => 1,
            NamedColor::DimGreen => 2,
            NamedColor::DimYellow => 3,
            NamedColor::DimBlue => 4,
            NamedColor::DimMagenta => 5,
            NamedColor::DimCyan => 6,
            NamedColor::DimWhite => 7,
            NamedColor::BrightForeground => 39,
            NamedColor::DimForeground => 39,
        };

        // Adjust for bright colors (8-15)
        let code = if color_code >= 8 {
            base_code + 60 + (color_code - 8)
        } else {
            base_code + color_code
        };

        format!("\x1b[{}m", code)
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_basic() {
        let mut row = Row::new(10);
        row[0].c = 'H';
        row[1].c = 'e';
        row[2].c = 'l';
        row[3].c = 'l';
        row[4].c = 'o';

        let grid = vec![row];
        let renderer = Renderer::new();
        let output = renderer.render(&grid, 0, 5); // Cursor at column 5, not over text

        // Check that the characters are in the output
        assert!(output.contains('H'));
        assert!(output.contains('e'));
        assert!(output.contains('l'));
        assert!(output.contains('o'));
    }

    #[test]
    fn test_render_with_cursor() {
        let mut row = Row::new(10);
        row[0].c = 'H';
        row[1].c = 'e';

        let grid = vec![row];
        let renderer = Renderer::new();
        let output = renderer.render(&grid, 0, 1);

        assert!(output.contains('█'));
    }

    #[test]
    fn test_render_with_line_numbers() {
        let row = Row::new(10);
        let grid = vec![row.clone(), row.clone(), row.clone()];

        let config = RenderConfig {
            show_line_numbers: true,
            ..Default::default()
        };
        let renderer = Renderer::with_config(config);
        let output = renderer.render(&grid, 0, 0);

        assert!(output.contains("0 |"));
        assert!(output.contains("1 |"));
        assert!(output.contains("2 |"));
    }
}
