use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::formatter::FieldFormatter;

pub struct BaseFormatter();

impl FieldFormatter for BaseFormatter {
    fn format_field(&self, field: &Field) -> TokenStream {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        quote! {
            {
                "name": stringify!(#name),
                "type": stringify!(#ty),
            }
        }
    }
}
