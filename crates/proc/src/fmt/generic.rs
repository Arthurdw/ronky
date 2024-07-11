use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::{fmt::base::get_path, formatter::FieldFormatter};

pub struct GenericFormatter();

impl FieldFormatter for GenericFormatter {
    fn format_field(&self, field: &Field) -> TokenStream {
        let name = field.ident.as_ref().unwrap().to_string();
        let path = get_path(field);
        let segment = path.segments.first().unwrap();

        let segment_name = segment.ident.to_string();

        let arguments = match &segment.arguments {
            syn::PathArguments::AngleBracketed(arguments) => arguments
                .args
                .iter()
                .map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => ty,
                    t => panic!("Expected a type argument for a generic type. Got: {:?}", t),
                })
                .map(|ty| match ty {
                    syn::Type::Path(path) => path.path.get_ident().unwrap().to_string(),
                    t => panic!("Expected a path argument for a generic type. Got: {:?}", t),
                })
                .collect::<Vec<_>>(),
            t => panic!(
                "Expected angle bracketed arguments for a generic type. Got: {:?}",
                t
            ),
        };

        println!("{:#?}", arguments);

        quote! {
            {
                "name": #name,
                "type": #segment_name,
                "generics": [#(#arguments),*]
            }
        }
    }
}
