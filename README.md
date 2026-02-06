# quick-rust

[![CI](https://github.com/mezensky/quick-rust/workflows/CI/badge.svg)](https://github.com/mezensky/quick-rust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)

> Quick start Rust project template with best practices

A minimal Rust project template demonstrating modern Rust best practices including testing, documentation, linting, and project structure.

## Features

- Rust 2021 Edition
- Unit testing examples
- Documentation comments
- Clippy linting configuration
- Pre-push git hooks
- Makefile for common tasks
- GitHub Actions CI/CD pipeline
- Comprehensive test coverage
- Cargo caching for faster builds

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
├── .github/
│   └── workflows/
│       └── ci.yml       # GitHub Actions CI pipeline
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

## Continuous Integration

The project uses GitHub Actions for CI/CD with the following checks:

- **Format**: Ensures code follows Rust formatting standards
- **Clippy**: Runs the Rust linter with strict warnings
- **Test**: Runs all unit and integration tests
- **Build**: Builds both debug and release versions

All checks must pass before merging pull requests.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run `make all` to ensure all checks pass
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

MIT License - See LICENSE file for details
