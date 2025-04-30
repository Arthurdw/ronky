use proc_macro::TokenStream;
use syn::Field;

mod base;

pub(crate) use base::BaseParser;

use super::attributes::fields::FieldArguments;

pub(crate) enum ParsedField<'a> {
    Required(&'a Field, TokenStream, Option<FieldArguments>),
    Optional(&'a Field, TokenStream, Option<FieldArguments>),
}

pub(crate) trait FieldParser {
    fn parse(field: &Field) -> Result<ParsedField<'_>, TokenStream>;
}
