use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::formatter::FieldFormatter;

pub struct VecFormatter();

impl FieldFormatter for VecFormatter {
    fn format_field(&self, field: &Field) -> TokenStream {
        let name = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        // TODO: get argument from type

        quote! {
            {
                "name": stringify!(#name),
                "type": stringify!(#ty),
            }
        }
    }
}
