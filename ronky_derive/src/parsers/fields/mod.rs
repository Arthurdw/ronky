use proc_macro::TokenStream;
use syn::Field;

mod base;

/// Re-export of the `BaseParser` for field parsing.
pub(crate) use base::BaseParser;

use super::attributes::fields::FieldArguments;

/// Represents a parsed field with its associated metadata.
///
/// # Variants
///
/// * `Required` - A required field with its `Field` definition, `TokenStream`, and optional `FieldArguments`.
/// * `Optional` - An optional field with its `Field` definition, `TokenStream`, and optional `FieldArguments`.
pub(crate) enum ParsedField<'a> {
    Required(&'a Field, TokenStream, Option<FieldArguments>),
    Optional(&'a Field, TokenStream, Option<FieldArguments>),
}

/// Trait for parsing fields into a `ParsedField` representation.
pub(crate) trait FieldParser {
    /// Parses a field and returns a `ParsedField` or an error `TokenStream`.
    ///
    /// # Arguments
    ///
    /// * `field` - A reference to a `Field` object to parse.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `ParsedField` on success or a `TokenStream` on failure.
    fn parse(field: &Field) -> Result<ParsedField<'_>, TokenStream>;
}
