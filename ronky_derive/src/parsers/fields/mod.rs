use proc_macro::TokenStream;
use syn::Field;

mod base;

pub(crate) use base::BaseParser;

pub(crate) enum ParsedField<'a> {
    Required(&'a Field, TokenStream),
    Optional(&'a Field, TokenStream),
}

pub(crate) trait FieldParser {
    fn parse<'a>(parent: &str, field: &'a Field) -> Result<ParsedField<'a>, TokenStream>;
}
