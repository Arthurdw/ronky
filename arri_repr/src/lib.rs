#![allow(clippy::multiple_crate_versions)]

//! # Arri Schema Representation Library
//!
//! This crate provides all the types and Rust representations required for working with the Arri schema.
//! It serves as the foundational library for defining and manipulating schema elements, metadata,
//! properties, and other constructs used in Arri schemas.
//!
//! The primary purpose of this crate is to enable seamless integration and schema building
//! for projects that rely on the Arri schema format. If you're looking for the crate that
//! utilizes these representations to build schemas, you are probably looking for the
//! [ronky](https://docs.rs/ronky) crate.
//!
//! ## Features
//! - Comprehensive type definitions for Arri schemas.
//! - Utilities for serialization and metadata handling.
//! - Handles all serialization and deserialization of objects and schemas. (WIP)
//!
//! ## Usage
//! Import the necessary schema types and utilities from this crate to define, manipulate,
//! serialize, or deserialize Arri schemas in your Rust projects.

mod elements;
mod empty;
mod r#enum;
mod exportable;
mod metadata;
mod properties;
mod r#ref;
mod serializable;
mod serializer;
mod tagged_union;
mod r#type;
mod values;

pub use arri_common::EnumTransformation;
pub use elements::ElementsSchema;
pub use empty::EmptySchema;
pub use r#enum::EnumSchema;
pub use exportable::Exportable;
pub use metadata::MetadataSchema;
pub use properties::PropertiesSchema;
pub use r#ref::RefSchema;
pub use serializable::Serializable;
pub use serializer::Serializer;
pub use tagged_union::TaggedUnionSchema;
pub use r#type::{TypeSchema, Types};
pub use values::ValuesSchema;

pub mod type_utils {
    /// Extracts a type name from a string representation.
    ///
    /// This function takes a string-like input, splits it into parts based on non-alphanumeric
    /// characters (excluding underscores), and filters out words that are empty or do not start
    /// with an uppercase letter. The remaining parts are concatenated into a single string.
    ///
    /// # Arguments
    ///
    /// * `repr` - An input that implements the `ToString` trait, representing the type name.
    ///
    /// # Returns
    ///
    /// A `String` containing the extracted type name.
    pub fn get_type_name_from(repr: impl ToString) -> String {
        repr.to_string()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|word| !word.is_empty() && word.chars().next().unwrap().is_uppercase())
            .collect::<String>()
    }

    /// Retrieves the type name of a given type `T`.
    ///
    /// This function uses Rust's `std::any::type_name` to get the fully qualified name of the type
    /// and then processes it using `get_type_name_from` to extract a simplified type name.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type whose name is to be retrieved. It can be any type, including unsized types.
    ///
    /// # Returns
    ///
    /// A `String` containing the extracted type name of `T`.
    pub fn get_type_name<T>() -> String
    where
        T: ?Sized,
    {
        get_type_name_from(std::any::type_name::<T>())
    }
}
