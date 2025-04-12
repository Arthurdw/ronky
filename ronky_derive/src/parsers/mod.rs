use arri_repr::{Serializable, TypeSchema, Types};
use proc_macro::TokenStream;
use quote::quote;
use syn::Field;

pub(crate) struct ParsedField {
    pub(crate) properties: Option<TokenStream>,
    pub(crate) optional_properties: Option<TokenStream>,
}

pub(crate) trait FieldParser {
    fn parse(&self, field: &Field) -> ParsedField;
}

pub fn parse_field(field: &Field) -> ParsedField {
    ParsedField {
        properties: Some(
            quote! {
                TypeSchema::new(Types::String)
            }
            .into(),
        ),
        optional_properties: None,
    }
}
