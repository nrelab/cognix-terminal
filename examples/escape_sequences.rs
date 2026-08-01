//! Escape sequences example
//! Demonstrates ANSI escape sequence handling

use cognix_terminal::ansi::escape_sequences::*;

fn main() {
    println!("=== Cognix Terminal Escape Sequences Demo ===\n");

    println!("--- C0 Control Characters ---");
    println!("NUL: 0x{:02X}", C0::NUL);
    println!("BEL: 0x{:02X}", C0::BEL);
    println!("BS:  0x{:02X}", C0::BS);
    println!("HT:  0x{:02X}", C0::HT);
    println!("LF:  0x{:02X}", C0::LF);
    println!("CR:  0x{:02X}", C0::CR);
    println!("ESC: 0x{:02X}", C0::ESC);

    println!("\n--- C1 Control Sequences ---");
    println!("CSI: {:?}", C1::CSI);
    println!("OSC: {:?}", C1::OSC);
    println!("ST:  {:?}", C1::ST);
    println!("DCS: {:?}", C1::DCS);

    println!("\n--- Bracketed Paste ---");
    println!("Start: {:?}", BRACKETED_PASTE_START);
    println!("End:   {:?}", BRACKETED_PASTE_END);

    println!("\n--- Arrow Keys ---");
    println!("Up:    0x{:02X}", EscCodes::ARROW_UP);
    println!("Down:  0x{:02X}", EscCodes::ARROW_DOWN);
    println!("Right: 0x{:02X}", EscCodes::ARROW_RIGHT);
    println!("Left:  0x{:02X}", EscCodes::ARROW_LEFT);

    println!("\n--- Building Escape Sequences ---");
    let seq = EscCodes::build_escape_sequence(C1::CSI, b'A');
    println!("CSI A (cursor up): {:?}", seq);

    let seq = EscCodes::build_escape_sequence(C1::CSI, b'B');
    println!("CSI B (cursor down): {:?}", seq);

    let seq = EscCodes::build_escape_sequence(C1::CSI, b'C');
    println!("CSI C (cursor forward): {:?}", seq);

    let seq = EscCodes::build_escape_sequence(C1::CSI, b'D');
    println!("CSI D (cursor back): {:?}", seq);

    println!("\n--- Mouse Codes ---");
    println!("Left: {}", EscCodes::MOUSE_LEFT);
    println!("Right: {}", EscCodes::MOUSE_RIGHT);
    println!("Drag: {}", EscCodes::MOUSE_DRAG);
    println!("Move: {}", EscCodes::MOUSE_MOVE);
    println!("Wheel Up: {}", EscCodes::MOUSE_WHEEL_UP);
    println!("Wheel Down: {}", EscCodes::MOUSE_WHEEL_DOWN);

    println!("\n--- Focus Events ---");
    println!("Focus In: {:?}", EscCodes::FOCUS_IN);
    println!("Focus Out: {:?}", EscCodes::FOCUS_OUT);

    println!("\n=== Demo Complete ===");
}
