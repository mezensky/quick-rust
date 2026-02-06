# quick-rust

> Quick start Rust project template with best practices

A minimal Rust project template demonstrating modern Rust best practices including testing, documentation, linting, and project structure.

## Features

- Rust 2021 Edition
- Unit testing examples
- Documentation comments
- Clippy linting configuration
- Pre-push git hooks
- Makefile for common tasks
- CI/CD ready structure

## Quick Start

```bash
# Install dependencies
make install

# Run the project
make run

# Run tests
make test

# Format and lint
make fix

# Run all checks (format, lint, test)
make all
```

## Project Structure

```
quick-rust/
├── src/
│   └── main.rs          # Main application code with examples
├── hooks/
│   └── pre-push         # Git pre-push hook
├── Cargo.toml           # Project manifest
├── Makefile             # Build automation
├── CLAUDE.md            # Claude Code instructions
├── CHANGELOG.md         # Project changelog
└── clippy.toml          # Linting configuration
```

## Development

### Available Make Commands

- `make install` - Check and install dependencies
- `make run` - Run the application
- `make test` - Run all tests
- `make fmt-fix` - Format code
- `make lint` - Run clippy linter
- `make fix` - Format and lint
- `make all` - Run all checks
- `make build` - Build debug version
- `make build-prod` - Build release version
- `make clean` - Clean build artifacts

### Git Hooks

Install the pre-push hook to run checks before pushing:

```bash
make hooks
```

## Testing

Run tests with:

```bash
cargo test
# or
make test
```

## Documentation

Generate and view documentation:

```bash
cargo doc --open
```

## License

MIT License - See LICENSE file for details


