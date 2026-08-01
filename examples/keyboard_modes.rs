//! Terminal modes example
//! Demonstrates terminal mode flags and keyboard protocol modes

use cognix_terminal::prelude::*;
use cognix_terminal::mode::{KeyboardModes, KeyboardModesApplyBehavior};

fn main() {
    println!("=== Cognix Terminal Modes Demo ===\n");

    // Create terminal with default modes
    let mut terminal = Terminal::new(10, 40);
    
    println!("Default terminal modes:");
    println!("  SHOW_CURSOR: {}", terminal.mode().contains(TermMode::SHOW_CURSOR));
    println!("  LINE_WRAP: {}", terminal.mode().contains(TermMode::LINE_WRAP));
    println!("  BRACKETED_PASTE: {}", terminal.mode().contains(TermMode::BRACKETED_PASTE));
    
    // Set specific modes
    println!("\nSetting BRACKETED_PASTE mode...");
    terminal.set_mode(terminal.mode() | TermMode::BRACKETED_PASTE);
    println!("  BRACKETED_PASTE: {}", terminal.mode().contains(TermMode::BRACKETED_PASTE));
    
    // Toggle modes
    println!("\nToggling SHOW_CURSOR off...");
    terminal.set_mode(terminal.mode() & !TermMode::SHOW_CURSOR);
    println!("  SHOW_CURSOR: {}", terminal.mode().contains(TermMode::SHOW_CURSOR));
    
    // Keyboard protocol modes
    println!("\n--- Kitty Keyboard Protocol Modes ---");
    
    let mut keyboard_modes = KeyboardModes::NO_MODE;
    println!("Initial: {:?}", keyboard_modes);
    
    // Enable disambiguation
    keyboard_modes |= KeyboardModes::DISAMBIGUATE_ESC_CODES;
    println!("After DISAMBIGUATE_ESC_CODES: {:?}", keyboard_modes);
    
    // Enable event type reporting
    keyboard_modes |= KeyboardModes::REPORT_EVENT_TYPES;
    println!("After REPORT_EVENT_TYPES: {:?}", keyboard_modes);
    
    // Enable alternate keys
    keyboard_modes |= KeyboardModes::REPORT_ALTERNATE_KEYS;
    println!("After REPORT_ALTERNATE_KEYS: {:?}", keyboard_modes);
    
    // Convert to TermMode
    let term_mode: TermMode = keyboard_modes.into();
    println!("\nConverted to TermMode:");
    println!("  KEYBOARD_DISAMBIGUATE_ESCAPE: {}", 
             term_mode.contains(TermMode::KEYBOARD_DISAMBIGUATE_ESCAPE));
    println!("  KEYBOARD_REPORT_EVENT_TYPES: {}", 
             term_mode.contains(TermMode::KEYBOARD_REPORT_EVENT_TYPES));
    println!("  KEYBOARD_REPORT_ALTERNATE_KEYS: {}", 
             term_mode.contains(TermMode::KEYBOARD_REPORT_ALTERNATE_KEYS));
    
    // Apply behavior examples
    println!("\n--- Keyboard Modes Apply Behavior ---");
    let _base_modes = KeyboardModes::DISAMBIGUATE_ESC_CODES;
    
    let replace = KeyboardModesApplyBehavior::Replace;
    println!("Replace behavior: {:?}", replace);
    
    let union = KeyboardModesApplyBehavior::Union;
    println!("Union behavior: {:?}", union);
    
    let difference = KeyboardModesApplyBehavior::Difference;
    println!("Difference behavior: {:?}", difference);
    
    println!("\n=== Demo Complete ===");
}
