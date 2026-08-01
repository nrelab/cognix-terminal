//! Cursor manipulation example
//! Demonstrates cursor movement and positioning

use cognix_terminal::prelude::*;
use cognix_terminal::render::Renderer;

fn main() {
    println!("=== Cognix Terminal Cursor Demo ===\n");

    // Create terminal
    let mut terminal = Terminal::new(10, 40);

    // Write some text
    terminal.write_str("Hello, World!");

    println!(
        "Initial cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Move cursor around
    println!("\nMoving cursor to (2, 5)...");
    terminal.move_cursor(2, 5);
    terminal.write_str("Text at (2,5)");

    println!(
        "Cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Cursor forward
    println!("\nMoving cursor forward 10 positions...");
    terminal.cursor_forward(10);
    println!(
        "Cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Cursor backward
    println!("\nMoving cursor backward 5 positions...");
    terminal.cursor_backward(5);
    println!(
        "Cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Cursor down
    println!("\nMoving cursor down 2 lines...");
    terminal.cursor_down(2);
    println!(
        "Cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Cursor up
    println!("\nMoving cursor up 1 line...");
    terminal.cursor_up(1);
    println!(
        "Cursor position: ({}, {})",
        terminal.cursor().point.row,
        terminal.cursor().point.col
    );

    // Render the terminal
    println!("\n--- Terminal Output ---");
    let renderer = Renderer::new();
    let output = renderer.render(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);

    println!("=== Demo Complete ===");
}
