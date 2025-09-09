# Contributing to Ronky

Thank you for your interest in contributing to Ronky!

## Development Setup

1. Clone the repository
2. Install pre-commit hooks: `prek install`
3. Run tests: `cargo nextest run --all`

## Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) for our commit messages.

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **build**: Changes that affect the build system or external dependencies
- **ci**: Changes to CI configuration files and scripts
- **chore**: Other changes that don't modify src or test files
- **revert**: Reverts a previous commit

### Examples

```
feat: add json serialization support

fix: correct enum serialization for tagged unions

docs: update readme with serialization examples

refactor: simplify exportable trait implementation

test: add tests for nested struct serialization
```

### Scope (Optional)

The scope could be anything specifying the place of the commit change:
- `ronky`
- `ronky_derive`
- `arri_repr`
- `examples`
- `tests`

### Rules

1. Use lowercase for the subject
2. Don't end the subject with a period
3. Keep the subject line under 72 characters
4. Use imperative mood ("add" not "added" or "adds")
5. Separate body from subject with a blank line
6. Wrap body at 100 characters
7. Use body to explain what and why, not how

## Pull Request Process

1. Ensure all tests pass: `cargo nextest run --all`
2. Run formatting: `cargo fmt --all`
3. Fix clippy warnings: `cargo clippy --all-features --all-targets --fix`
4. Update documentation if needed
5. Add tests for new features
6. Use conventional commits for all commits
7. Keep PRs focused on a single feature or fix

## Code Quality

Pre-commit hooks will automatically:
- Format code with rustfmt
- Fix clippy warnings where possible
- Remove trailing whitespace
- Ensure files end with newline
- Validate YAML and TOML files

GitHub Actions will:
- Run tests on multiple platforms (Linux, Windows, macOS)
- Check formatting and linting
- Automatically fix issues in PRs when possible
- Validate commit messages

## Testing

- Use `cargo nextest run` for faster test execution
- Write tests for all new functionality
- Ensure tests are deterministic and don't depend on external state
- Use property-based testing where appropriate

## Documentation

- Document all public APIs
- Include examples in doc comments
- Update README for significant features
- Keep CHANGELOG up to date

## Questions?

Feel free to open an issue for any questions about contributing!
