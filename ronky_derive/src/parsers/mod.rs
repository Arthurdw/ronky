pub(crate) mod attributes;
mod fields;
mod types;

use fields::{BaseParser, FieldParser};
use proc_macro::TokenStream;
use syn::Field;

pub(crate) use fields::ParsedField;

// TODO: docs
pub fn parse_field(field: &Field) -> Result<ParsedField<'_>, TokenStream> {
    BaseParser::parse(field)
}
