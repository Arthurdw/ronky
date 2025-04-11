use arri_repr::{Serializable, TypeSchema, Types};
use proc_macro::TokenStream;
use quote::quote;
use syn::Field;

pub(crate) trait FieldParser {
    fn parse(&self, field: &Field) -> TokenStream;
}

pub fn parse_field(field: &Field) -> TokenStream {
    quote! {
            TypeSchema::new(Types::String)
    }
    .into()
}
