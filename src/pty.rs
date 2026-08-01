//! PTY I/O layer for shell integration
//! Provides basic PTY spawning and I/O handling

#![allow(clippy::disallowed_types)]

use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

/// PTY handle for shell communication
#[allow(clippy::disallowed_types)]
pub struct Pty {
    /// Child process handle
    child: Option<std::process::Child>,
    /// Write handle for sending input to shell
    write: Option<Box<dyn Write + Send>>,
    /// Read handle for receiving output from shell
    read: Option<Box<dyn Read + Send>>,
}

impl Pty {
    /// Spawn a new PTY with the default shell
    #[cfg(unix)]
    #[allow(clippy::disallowed_types)]
    pub fn spawn_shell() -> io::Result<Self> {
        // On Unix systems, we would typically use a PTY library like `pty`
        // For this minimal implementation, we'll use pipes
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        
        let mut cmd = Command::new(&shell);
        cmd.stdin(Stdio::piped())
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let mut child = cmd.spawn()?;
        
        let write = child.stdin.take().map(|stdin| {
            Box::new(stdin) as Box<dyn Write + Send>
        });
        
        let read = child.stdout.take().map(|stdout| {
            Box::new(stdout) as Box<dyn Read + Send>
        });
        
        Ok(Self {
            child: Some(child),
            write,
            read,
        })
    }

    /// Spawn a new PTY with the default shell (Windows stub)
    #[cfg(windows)]
    pub fn spawn_shell() -> io::Result<Self> {
        // Windows PTY support requires conpty or similar
        // For now, return a stub implementation
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "PTY not yet implemented on Windows",
        ))
    }

    /// Write bytes to the PTY
    pub fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if let Some(ref mut write) = self.write {
            write.write(bytes)
        } else {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "PTY write handle closed"))
        }
    }

    /// Read bytes from the PTY (non-blocking, returns what's available)
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if let Some(ref mut read) = self.read {
            read.read(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "PTY read handle closed"))
        }
    }

    /// Flush the write buffer
    pub fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut write) = self.write {
            write.flush()
        } else {
            Ok(())
        }
    }

    /// Check if the PTY is still alive
    pub fn is_alive(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            match child.try_wait() {
                Ok(Some(_)) => false, // Process has exited
                Ok(None) => true,     // Still running
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Resize the PTY (stub implementation)
    pub fn resize(&mut self, _rows: u16, _cols: u16) -> io::Result<()> {
        // Would send SIGWINCH or equivalent on real PTY
        Ok(())
    }
}

impl Drop for Pty {
    fn drop(&mut self) {
        // Try to gracefully terminate the child process
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(unix)]
    fn test_pty_spawn() {
        let pty = Pty::spawn_shell();
        assert!(pty.is_ok());
        
        let mut pty = pty.unwrap();
        assert!(pty.is_alive());
    }

    #[test]
    #[cfg(unix)]
    fn test_pty_write() {
        let mut pty = Pty::spawn_shell().unwrap();
        let result = pty.write(b"echo hello\n");
        assert!(result.is_ok());
    }
}
