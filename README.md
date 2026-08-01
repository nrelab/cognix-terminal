# cognix-terminal

A terminal emulation library extracted from Warp, providing VTE/ANSI parsing, grid state management, and terminal mode handling.

## Overview

`cognix-terminal` is a lightweight, standalone terminal emulation library that provides the core functionality needed to build terminal emulators, terminal-based applications, or integrate terminal emulation into existing projects. It extracts the essential terminal emulation components from Warp's codebase while removing Warp-specific dependencies.

## Features

- **VTE/ANSI Parsing**: Full support for ANSI escape sequences and control codes
  - C0/C1 control characters
  - CSI (Control Sequence Introducer) sequences
  - OSC (Operating System Command) sequences
  - Bracketed paste support
  - Kitty keyboard protocol

- **Grid State Management**: Efficient cell and row management for terminal display
  - Memory-optimized cell representation (24 bytes per cell)
  - Dirty tracking for efficient rendering
  - Zero-width character support
  - Hyperlink tracking

- **Terminal Modes**: Comprehensive mode flags for cursor, mouse, and keyboard protocols
  - Terminal mode bitflags (cursor visibility, line wrap, etc.)
  - Kitty keyboard protocol modes
  - Mouse reporting modes

- **PTY Integration**: Basic PTY spawning and I/O handling
  - Shell spawning on Unix systems
  - Read/write operations for PTY communication
  - Process lifecycle management

- **Rendering**: Text-based rendering with optional ANSI color support
  - Plain text rendering
  - ANSI color rendering
  - Line number display
  - Cursor position visualization

## Architecture

The library is organized into several core modules:

### Core Modules

- **`ansi`**: VTE/ANSI parsing and control sequence handling
  - `control_sequence_parameters`: Terminal attributes, colors, cursor styles, SGR parameters
  - `escape_sequences`: C0/C1 control characters and escape sequence definitions

- **`grid`**: Terminal grid state management
  - `cell`: Individual cell representation with memory optimization
  - `row`: Row management with dirty tracking

- **`mode`**: Terminal mode management
  - `TermMode`: Bitflags for terminal modes (cursor, mouse, keyboard protocols)
  - `KeyboardModes`: Kitty keyboard protocol support

- **`indexing`**: Grid indexing utilities
  - `Point`: Row/column positioning
  - `VisiblePoint`: Visible row/column positioning

- **`terminal`**: Terminal model coordinator
  - `Terminal`: Main terminal emulation implementation
  - `Size`, `Cursor`: Terminal dimensions and cursor state

- **`pty`**: PTY I/O layer
  - `Pty`: Shell spawning and I/O handling

- **`render`**: Rendering interface
  - `Renderer`: Text-based terminal rendering

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
cognix-terminal = "0.1.0"
```

## Usage

### Basic Terminal Emulation

```rust
use cognix_terminal::prelude::*;

// Create a terminal with 24 rows and 80 columns
let mut terminal = Terminal::new(24, 80);

// Write text
terminal.write_str("Hello, World!\n");

// Set colors
terminal.set_fg_color(Color::Named(NamedColor::Green));
terminal.write_str("Green text\n");

// Move cursor
terminal.move_cursor(10, 10);
terminal.write_str("Cursor at (10, 10)");

// Render
let renderer = Renderer::new();
let output = renderer.render(
    terminal.grid(),
    terminal.cursor().point.row,
    terminal.cursor().point.col,
);
println!("{}", output);
```

### Terminal Modes

```rust
use cognix_terminal::prelude::*;

let mut terminal = Terminal::new(24, 80);

// Enable bracketed paste mode
terminal.set_mode(terminal.mode() | TermMode::BRACKETED_PASTE);

// Disable cursor
terminal.set_mode(terminal.mode() & !TermMode::SHOW_CURSOR);

// Check mode status
if terminal.mode().contains(TermMode::LINE_WRAP) {
    println!("Line wrap is enabled");
}
```

### Color Management

```rust
use cognix_terminal::prelude::*;
use cognix_terminal::ansi::control_sequence_parameters::ColorU;

// Named colors
terminal.set_fg_color(Color::Named(NamedColor::Red));

// RGB colors
terminal.set_fg_color(Color::Spec(ColorU::new(255, 100, 100, 255)));

// Indexed colors (256-color palette)
terminal.set_fg_color(Color::Indexed(208));

// Reset to defaults
terminal.reset_colors();
```

### PTY Integration

```rust
use cognix_terminal::prelude::*;

// Spawn a shell
let mut pty = Pty::spawn_shell()?;

// Send commands
pty.write(b"echo hello\n")?;
pty.flush()?;

// Read output
let mut buffer = [0u8; 4096];
let n = pty.read(&mut buffer)?;

// Feed to terminal
terminal.input(&buffer[..n]);
```

## Running Examples

```bash
# Basic terminal demo
cargo run --example basic

# PTY integration demo (Unix only)
cargo run --example pty_demo

# Color management demo
cargo run --example colors

# Keyboard modes demo
cargo run --example keyboard_modes

# Escape sequences demo
cargo run --example escape_sequences

# Cursor manipulation demo
cargo run --example cursor
```

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_terminal_new
```

## Documentation

Build the documentation:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please read CONTRIBUTING.md for details on our code of conduct and the process for submitting pull requests.

## License

This code is adapted from Warp's terminal emulation layer, which is licensed under Apache-2.0 and MIT. This library is dual-licensed under MIT OR Apache-2.0 at your option.

## Acknowledgments

- **Warp**: Original source of the terminal emulation code
- **Alacritty**: VTE parsing and grid management code (Apache-2.0 licensed)
- **vte crate**: ANSI escape sequence parsing (Apache-2.0 licensed)

## Project Status

This is an extraction of Warp's terminal emulation core. The library provides a solid foundation for terminal emulation but may not include all features present in the full Warp application. Future development may include:

- Enhanced VTE parsing
- More comprehensive mouse protocol support
- Performance optimizations
- Additional rendering backends
