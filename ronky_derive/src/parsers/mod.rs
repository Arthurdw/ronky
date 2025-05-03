pub(crate) mod attributes;

mod fields;
mod types;
use fields::{BaseParser, FieldParser};
use proc_macro::TokenStream;
use syn::Field;

/// Re-export of the `ParsedField` type from the `fields` module.
pub(crate) use fields::ParsedField;

/// Parses a single field and returns a `ParsedField` representation.
///
/// # Arguments
///
/// * `field` - A reference to a `Field` object to be parsed.
///
/// # Returns
///
/// Returns a `Result` containing a `ParsedField` on success or a `TokenStream` on failure.
pub fn parse_field(field: &Field) -> Result<ParsedField<'_>, TokenStream> {
    BaseParser::parse(field)
}
