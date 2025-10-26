# Contributing to Conversions RS

Thank you for your interest in contributing to Conversions RS! We welcome contributions from the community.

## Development Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/RK1PF/conversions_rs.git
   cd conversions_rs
   ```

2. **Install Rust:**
   If you don't have Rust installed, visit [rustup.rs](https://rustup.rs/) to install it.

3. **Build the project:**
   ```bash
   cargo build
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

5. **Run the application:**
   ```bash
   cargo run
   ```

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check if the issue already exists. When you create a bug report, please include as many details as possible:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Provide specific examples to demonstrate the steps
- Describe the behavior you observed and what behavior you expected
- Include your environment details (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- Use a clear and descriptive title
- Provide a step-by-step description of the suggested enhancement
- Provide specific examples to demonstrate the enhancement
- Describe the current behavior and explain the behavior you expected to see
- Explain why this enhancement would be useful

### Pull Requests

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes if applicable
5. Ensure all tests pass (`cargo test`)
6. Ensure your code follows the project's style guidelines (`cargo fmt` and `cargo clippy`)
7. Commit your changes (`git commit -m 'Add some amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Code Style

- Run `cargo fmt` to format your code
- Run `cargo clippy` to check for common mistakes
- Follow Rust naming conventions
- Add documentation comments for public APIs
- Keep functions small and focused
- Write meaningful commit messages

### Adding New Unit Conversions

When adding support for new units:

1. Add the unit to the appropriate module in `src/conversions/`
2. Update the corresponding conversion functions
3. Add comprehensive tests
4. Update the help text and documentation
5. Add examples to the README if applicable

### Testing

- Write unit tests for all new functionality
- Ensure existing tests continue to pass
- Test both the library API and CLI interface
- Include edge cases in your tests

### Documentation

- Update README.md if you change functionality
- Add or update inline documentation for new/changed functions
- Ensure examples in documentation are correct and up-to-date

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on what is best for the community
- Be collaborative
- Be mindful of your language and behavior

## Questions?

If you have questions about contributing, feel free to:
- Open an issue with the "question" label
- Start a discussion in the repository

## License

By contributing to Conversions RS, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).