//! Color management example
//! Demonstrates terminal color handling and ANSI color codes

use cognix_terminal::prelude::*;
use cognix_terminal::ansi::control_sequence_parameters::ColorU;
use cognix_terminal::render::{Renderer, RenderConfig};

fn main() {
    println!("=== Cognix Terminal Color Demo ===\n");

    // Create a terminal
    let mut terminal = Terminal::new(15, 60);
    
    // Write text with different colors
    println!("Setting foreground colors...");
    
    // Standard ANSI colors
    terminal.set_fg_color(Color::Named(NamedColor::Red));
    terminal.write_str("Red text ");
    
    terminal.set_fg_color(Color::Named(NamedColor::Green));
    terminal.write_str("Green text ");
    
    terminal.set_fg_color(Color::Named(NamedColor::Blue));
    terminal.write_str("Blue text ");
    
    terminal.set_fg_color(Color::Named(NamedColor::Yellow));
    terminal.write_str("Yellow text\n");
    
    // Bright colors
    terminal.set_fg_color(Color::Named(NamedColor::BrightRed));
    terminal.write_str("Bright Red ");
    
    terminal.set_fg_color(Color::Named(NamedColor::BrightGreen));
    terminal.write_str("Bright Green ");
    
    terminal.set_fg_color(Color::Named(NamedColor::BrightBlue));
    terminal.write_str("Bright Blue ");
    
    terminal.set_fg_color(Color::Named(NamedColor::BrightYellow));
    terminal.write_str("Bright Yellow\n");
    
    // RGB colors
    terminal.set_fg_color(Color::Spec(ColorU::new(255, 100, 100, 255)));
    terminal.write_str("Custom RGB color (255, 100, 100) ");
    
    terminal.set_fg_color(Color::Spec(ColorU::new(100, 255, 100, 255)));
    terminal.write_str("Custom RGB color (100, 255, 100)\n");
    
    // Indexed colors (256-color palette)
    terminal.set_fg_color(Color::Indexed(208));
    terminal.write_str("Indexed color 208 (orange) ");
    
    terminal.set_fg_color(Color::Indexed(141));
    terminal.write_str("Indexed color 141 (purple)\n");
    
    // Reset to default
    terminal.reset_colors();
    terminal.write_str("Back to default color\n");
    
    // Background colors
    terminal.move_cursor(terminal.cursor().point.row + 1, 0);
    terminal.set_bg_color(Color::Named(NamedColor::Red));
    terminal.set_fg_color(Color::Named(NamedColor::White));
    terminal.write_str("Red background with white text\n");
    
    terminal.reset_colors();
    terminal.write_str("Reset colors\n");
    
    // Render with ANSI color codes
    println!("\n--- Rendered with ANSI colors ---");
    let config = RenderConfig {
        use_colors: true,
        show_cursor: false,
        ..Default::default()
    };
    let renderer = Renderer::with_config(config);
    let output = renderer.render_colored(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);
    
    println!("\n=== Demo Complete ===");
}
