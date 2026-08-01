# Contributing to cognix-terminal

Thank you for your interest in contributing to cognix-terminal! This document provides guidelines for contributing to the project.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/cognix-terminal.git
   cd cognix-terminal
   ```

2. **Install Rust**
   Ensure you have Rust installed. The project uses Rust 2024 edition.
   ```bash
   rustup update
   ```

3. **Run tests**
   ```bash
   cargo test
   ```

4. **Run examples**
   ```bash
   cargo run --example basic
   ```

## Coding Standards

### Style

- Use `cargo fmt` to format code
- Follow Rust naming conventions
- Keep functions focused and small
- Add documentation comments (`///`) for public APIs

### Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting
- Use descriptive test names
- Test edge cases and error conditions

### Documentation

- Document public APIs with `///` comments
- Include usage examples in documentation
- Update README.md for user-facing changes
- Update CHANGELOG.md for significant changes

## Pull Request Process

1. **Fork the repository**
2. **Create a branch** for your feature or bugfix
3. **Make your changes** following the coding standards
4. **Run tests** to ensure everything works
5. **Commit your changes** with clear, descriptive messages
6. **Push to your fork** and create a pull request

### Pull Request Guidelines

- Describe what your PR does
- Reference any related issues
- Include screenshots if applicable
- Ensure CI checks pass
- Request review from maintainers

## Project Structure

```
cognix-terminal/
├── src/
│   ├── ansi/           # VTE/ANSI parsing
│   ├── grid/           # Grid state management
│   ├── mode.rs         # Terminal modes
│   ├── indexing.rs     # Grid indexing
│   ├── terminal.rs     # Terminal model
│   ├── pty.rs          # PTY I/O
│   ├── render.rs       # Rendering
│   └── lib.rs          # Library entry point
├── examples/           # Example programs
├── .github/workflows/  # CI/CD
├── Cargo.toml          # Package configuration
└── README.md           # Project documentation
```

## Areas for Contribution

We welcome contributions in the following areas:

- **VTE Parsing**: Enhanced escape sequence support
- **Grid Management**: Performance optimizations
- **PTY Integration**: Windows support, better error handling
- **Rendering**: Additional rendering backends
- **Documentation**: Examples, tutorials, API docs
- **Testing**: Test coverage, integration tests
- **Bug Fixes**: Any issues you encounter

## Reporting Issues

When reporting issues, please include:

- Rust version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
- Relevant code snippets

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.

## Questions?

Feel free to open an issue for questions or discussion.
