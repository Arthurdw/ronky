use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::formatter::FieldFormatter;

pub struct BaseFormatter();

impl FieldFormatter for BaseFormatter {
    fn format_field(&self, field: &Field) -> TokenStream {
        let name = field.ident.as_ref().unwrap().to_string();
        let ty = &field.ty;

        quote! {
            {
                "name": #name,
                "type": stringify!(#ty).to_lowercase(),
            }
        }
    }
}

pub fn get_path(field: &Field) -> syn::Path {
    match &field.ty {
        syn::Type::Path(path) => path.path.clone(),
        t => panic!("Received unsupported type! {:?}", t),
    }
}

pub fn fmt_ident(ident: &syn::Ident) -> String {
    ident.to_string().to_lowercase()
}
