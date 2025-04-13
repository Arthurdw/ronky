pub(crate) mod attributes;
mod fields;
mod types;

use fields::{BaseParser, FieldParser};
use syn::Field;

pub(crate) use fields::ParsedField;

// TODO: docs
pub fn parse_field<'a>(field: &'a Field) -> ParsedField<'a> {
    BaseParser::parse(field)
}
