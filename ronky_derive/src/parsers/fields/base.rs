use syn::Field;

use crate::parsers::types::{is_option_type, parse_type};

use super::{FieldParser, ParsedField};

pub struct BaseParser;

impl FieldParser for BaseParser {
    fn parse<'a>(field: &'a Field) -> ParsedField<'a> {
        let parsed = parse_type(&field.ty);

        if is_option_type(&field.ty) {
            return ParsedField::Optional(field, parsed);
        }
        ParsedField::Required(field, parsed)
    }
}
