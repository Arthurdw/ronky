# Arri Common

Common types and utilities shared across Arri crates.

This crate contains shared definitions to avoid circular dependencies and code duplication between `arri_repr` and `ronky_derive`.

## Features

- `EnumTransformation` - Shared enum for string case transformations using the `heck` crate

## Usage

This crate is primarily intended as an internal dependency for other Arri crates. It helps eliminate circular dependencies while providing a single source of truth for common types.
