//! PTY integration example
//! Demonstrates spawning a shell and interacting with it

use std::io;

#[cfg(unix)]
use cognix_terminal::prelude::*;
#[cfg(unix)]
use cognix_terminal::render::Renderer;
#[cfg(unix)]
use std::thread;
#[cfg(unix)]
use std::time::Duration;

#[cfg(unix)]
fn main() -> io::Result<()> {
    println!("=== Cognix Terminal PTY Demo ===\n");

    // Create terminal
    let mut terminal = Terminal::new(24, 80);

    // Spawn PTY shell
    println!("Spawning shell...");
    let mut pty = Pty::spawn_shell()?;

    // Send some commands to the shell
    println!("Sending commands to shell...");
    pty.write(b"echo 'Hello from cognix-terminal!'\n")?;
    pty.write(b"pwd\n")?;
    pty.write(b"ls\n")?;
    pty.write(b"echo 'Demo complete'\n")?;
    pty.flush()?;

    // Read output from shell
    println!("Reading shell output...");
    let mut buffer = [0u8; 4096];

    // Give the shell time to process
    thread::sleep(Duration::from_millis(500));

    // Read available output
    match pty.read(&mut buffer) {
        Ok(n) => {
            let output = &buffer[..n];
            println!("Received {} bytes from shell", n);

            // Feed output to terminal
            terminal.input(output);
        }
        Err(e) => {
            println!("Error reading from PTY: {}", e);
        }
    }

    // Render terminal
    println!("\n--- Terminal Output ---");
    let renderer = Renderer::new();
    let output = renderer.render(
        terminal.grid(),
        terminal.cursor().point.row,
        terminal.cursor().point.col,
    );
    println!("{}", output);

    // Check if shell is still alive
    println!("\nShell alive: {}", pty.is_alive());

    Ok(())
}

#[cfg(not(unix))]
fn main() -> io::Result<()> {
    println!("PTY demo is only supported on Unix systems.");
    Ok(())
}
