# Auto-Fixing with Pre-commit Hooks

This document explains what issues are automatically fixed by the pre-commit hooks.

## ✅ Issues That Are Automatically Fixed

### 1. **Rust Formatting** (`cargo fmt`)
- **What it fixes**: All formatting issues according to rustfmt rules
- **Examples**:
  - Incorrect indentation
  - Line length violations
  - Spacing around operators
  - Brace placement
  - Import ordering

### 2. **Clippy Warnings** (`cargo clippy --fix`)
- **What it fixes**: Many clippy warnings that have automatic fixes
- **Examples**:
  - Redundant clones
  - Unnecessary borrows
  - Simplifiable expressions
  - Redundant field names in struct initialization
  - Converting `&String` to `&str`
  - Using `writeln!` instead of `write!` with newline

### 3. **Trailing Whitespace**
- **What it fixes**: Removes all trailing spaces at the end of lines
- **Applies to**: All text files

### 4. **End of File**
- **What it fixes**: Ensures all files end with exactly one newline
- **Applies to**: All text files

### 5. **Line Endings**
- **What it fixes**: Converts all line endings to LF (Unix-style)
- **Applies to**: All text files

## ❌ Issues That Are NOT Automatically Fixed

### 1. **Compilation Errors**
- These require manual intervention
- The hooks will fail and prevent commit

### 2. **Test Failures**
- Tests must pass, no auto-fixing available
- Run on push, not commit

### 3. **Complex Clippy Warnings**
- Some clippy warnings require human judgment
- Examples:
  - API design issues
  - Performance considerations requiring algorithm changes
  - Security-related warnings

### 4. **Merge Conflicts**
- Detected but not fixed
- Must be resolved manually

## 🚀 Running Auto-Fix Manually

If you want to run auto-fixing manually before committing:

```bash
# Fix all formatting issues
cargo fmt --all

# Fix clippy warnings
cargo clippy --all-features --all-targets --fix --allow-dirty

# Run all pre-commit hooks with auto-fix (prek uses the same config as pre-commit)
prek run --all-files

# Or with pre-commit
pre-commit run --all-files
```

## 🔧 Customizing Auto-Fix Behavior

### Disable Auto-Fix for Specific Files

Add to `.gitignore` or use `exclude` patterns in `.pre-commit-config.yaml`:

```yaml
exclude: |
  (?x)^(
    path/to/excluded/file\.rs|
    another/excluded/.*
  )$
```

### Skip Hooks Temporarily

```bash
# Skip all hooks
git commit --no-verify

# Skip specific hook (not recommended)
SKIP=cargo-fmt git commit
```

## 📝 Best Practices

1. **Let hooks auto-fix before committing**: The hooks will automatically fix issues when you run `git commit`

2. **Review auto-fixes**: After hooks run, review the changes with `git diff` before finalizing the commit

3. **Run hooks before pushing**: Use `prek run --all-files` to catch issues early

4. **Don't skip hooks**: If a hook fails, fix the issue rather than bypassing it

5. **Keep dependencies updated**: Ensure rustfmt and clippy are up to date for best results
