use proc_macro::TokenStream;
use syn::Field;

mod base;

pub(crate) use base::BaseParser;

pub(crate) enum ParsedField<'a> {
    Required(&'a Field, TokenStream),
    Optional(&'a Field, TokenStream),
}

pub(crate) trait FieldParser {
    fn parse(field: &Field) -> Result<ParsedField, TokenStream>;
}
