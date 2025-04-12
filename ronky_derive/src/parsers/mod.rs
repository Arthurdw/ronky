use proc_macro::TokenStream;
use quote::quote;
use syn::Field;

pub(crate) enum ParsedField<'a> {
    Required(&'a Field, TokenStream),
    Optional(&'a Field, TokenStream),
}

pub(crate) trait FieldParser {
    fn parse(&self, field: &Field) -> ParsedField;
}

pub fn parse_field<'a>(field: &'a Field) -> ParsedField<'a> {
    let field_name = field.ident.as_ref().unwrap().to_string();
    println!("Processing field: {}", field_name);
    ParsedField::Optional(
        field,
        quote! {
            ronky::TypeSchema::new(ronky::Types::String)
        }
        .into(),
    )
}
