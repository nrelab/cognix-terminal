//! Basic terminal emulation example
//! Demonstrates the core functionality of cognix-terminal

use cognix_terminal::prelude::*;
use cognix_terminal::render::{RenderConfig, Renderer};

fn main() {
    println!("=== Cognix Terminal Demo ===\n");

    // Create a terminal with 24 rows and 80 columns
    let mut terminal = Terminal::new(24, 80);

    // Write some text to the terminal
    println!("Writing text to terminal...");
    terminal.write_str("Hello, World!\n");
    terminal.write_str("This is a demonstration of cognix-terminal.\n");
    terminal.write_str("A terminal emulation library extracted from Warp.\n");

    // Set some colors
    terminal.set_fg_color(Color::Named(NamedColor::Green));
    terminal.write_str("Green text\n");

    terminal.set_fg_color(Color::Named(NamedColor::Blue));
    terminal.write_str("Blue text\n");

    terminal.set_fg_color(Color::Named(NamedColor::Red));
    terminal.write_str("Red text\n");

    // Reset colors
    terminal.reset_colors();
    terminal.write_str("Normal text\n");

    // Move cursor around
    terminal.move_cursor(10, 10);
    terminal.write_str("Cursor at (10, 10)");

    // Render the terminal
    println!("\n--- Terminal Output ---");
    let renderer = Renderer::new();
    let output = renderer.render(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);

    // Demonstrate resize
    println!("--- Resizing to 15x40 ---");
    terminal.resize(15, 40);
    terminal.clear();
    terminal.write_str("Resized terminal!");

    let output = renderer.render(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);

    // Demonstrate with line numbers
    println!("--- With Line Numbers ---");
    terminal.resize(10, 30);
    terminal.clear();
    terminal.write_str("Line 1\nLine 2\nLine 3");

    let config = RenderConfig {
        show_line_numbers: true,
        ..Default::default()
    };
    let renderer = Renderer::with_config(config);
    let output = renderer.render(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);

    println!("\n=== Demo Complete ===");
}
