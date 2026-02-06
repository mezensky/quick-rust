# CLAUDE.md

This file contains project-specific instructions for Claude Code when working on this Rust project.

## Project Overview

This is a quick-start Rust project template demonstrating best practices for new Rust projects.

## Code Standards

- **Rust Edition**: 2021
- **Formatting**: Use `cargo fmt` before committing
- **Linting**: Code must pass `cargo clippy` without warnings
- **Testing**: All public functions should have unit tests
- **Documentation**: All public functions must have doc comments using `///`

## Development Workflow

1. Make changes to code
2. Format: `make fmt-fix`
3. Lint: `make lint`
4. Test: `make test`
5. Or run all: `make all`

## Testing Guidelines

- Add unit tests in the same file as the code being tested in a `#[cfg(test)]` module
- Integration tests go in the `tests/` directory
- Use descriptive test names: `test_<function_name>_<scenario>`
- Run tests with `cargo test` or `make test`

## Documentation

- Use `///` for public API documentation
- Include:
  - Brief description of what the function does
  - `# Arguments` section for parameters
  - `# Returns` section for return values
  - `# Examples` section with code examples
  - `# Panics` section if the function can panic
  - `# Errors` section for Result-returning functions

## Dependencies

- Keep dependencies minimal
- Prefer crate version specifications that allow patch updates (e.g., `"0.8"` instead of `"0.8.0"`)
- Document why each dependency is needed

## Code Style

- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Keep functions small and focused
- Prefer explicit types for public APIs
- Use `pub` only when necessary
- Prefer methods over functions when working with types

## When Making Changes

- Update CHANGELOG.md with notable changes
- Update tests when modifying functionality
- Update documentation when changing public APIs
- Run `make all` before considering work complete
