/// Common types and utilities shared across Arri crates.
///
/// This crate contains shared definitions to avoid circular dependencies
/// and code duplication between arri_repr and ronky_derive.
mod enum_transformation;

pub use enum_transformation::EnumTransformation;
