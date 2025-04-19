pub(crate) mod attributes;
mod fields;
mod types;

use fields::{BaseParser, FieldParser};
use proc_macro::TokenStream;
use syn::Field;

pub(crate) use fields::ParsedField;

// TODO: docs
pub fn parse_field<'a>(parent: &str, field: &'a Field) -> Result<ParsedField<'a>, TokenStream> {
    BaseParser::parse(parent, field)
}
