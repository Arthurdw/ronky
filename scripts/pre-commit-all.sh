#!/bin/bash
# Run all pre-commit checks

set -e

echo "Running cargo fmt..."
cargo fmt --all -- --check

echo "Running cargo clippy..."
cargo clippy --all-features --all-targets -- -D warnings

echo "Running cargo check..."
cargo check --all --all-features

echo "Running cargo tests..."
cargo nextest run --all

echo "All checks passed!"
