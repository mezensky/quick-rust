# Contributing to quick-rust

Thank you for your interest in contributing to quick-rust! This document provides guidelines and instructions for contributing.

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/quick-rust.git
   cd quick-rust
   ```
3. Set up git hooks:
   ```bash
   make hooks
   ```
4. Create a new branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Before You Start

1. Check existing issues and PRs to avoid duplication
2. For major changes, open an issue first to discuss
3. Ensure you have the latest Rust toolchain:
   ```bash
   rustup update
   ```

### Making Changes

1. Write your code following Rust conventions
2. Add tests for new functionality
3. Update documentation as needed
4. Update CHANGELOG.md under the `[Unreleased]` section

### Testing Your Changes

Run the full test suite:

```bash
# Run all checks (formatting, linting, tests)
make all

# Or run individually:
make fmt-fix    # Format code
make lint       # Run clippy
make test       # Run tests
make build      # Build project
```

All checks must pass before submitting a PR.

### Code Style

- Use `cargo fmt` for formatting
- Follow clippy suggestions: `cargo clippy`
- Write clear, self-documenting code
- Add doc comments (`///`) for public APIs
- Keep functions focused and small
- Write descriptive commit messages

### Commit Messages

Follow the conventional commits format:

```
<type>: <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:
```
feat: add support for custom random ranges

fix: correct off-by-one error in coin flip logic

docs: update README with new examples
```

### Pull Request Process

1. Ensure all tests pass and code is formatted
2. Update the README.md if needed
3. Update CHANGELOG.md with your changes
4. Fill out the PR template completely
5. Link any related issues
6. Wait for review and address feedback

### Review Process

- At least one maintainer approval is required
- All CI checks must pass
- Changes must be up-to-date with main branch
- Follow feedback and make requested changes
- Once approved, a maintainer will merge your PR

## Testing

### Unit Tests

Add tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        assert_eq!(my_function(), expected_value);
    }
}
```

### Integration Tests

Add integration tests in the `tests/` directory.

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with coverage (if installed)
cargo tarpaulin
```

## Documentation

### Doc Comments

Use `///` for public items:

```rust
/// Generates a random number.
///
/// # Returns
///
/// A random u8 value.
///
/// # Examples
///
/// ```
/// let num = generate_random_u8();
/// assert!(num <= 255);
/// ```
#[must_use]
pub fn generate_random_u8() -> u8 {
    random()
}
```

### Building Docs

```bash
cargo doc --open
```

## Need Help?

- Check existing documentation
- Review closed issues and PRs
- Open an issue with your question
- Tag it with the `question` label

## Recognition

Contributors will be recognized in:
- The project's README
- Release notes
- CHANGELOG.md

Thank you for contributing! 🦀
