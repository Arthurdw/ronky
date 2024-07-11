use proc_macro2::TokenStream;
use quote::quote;
use syn::Field;

use crate::{fmt::base::get_path, formatter::FieldFormatter};

pub struct VecFormatter();

impl FieldFormatter for VecFormatter {
    fn format_field(&self, field: &Field) -> TokenStream {
        let name = field.ident.as_ref().unwrap();
        let path = get_path(field);
        let segment = path.segments.first().unwrap();

        let segment_name = segment.ident.to_string();
        if segment_name != "Vec" {
            panic!("Expected Vec, found {}", segment_name);
        }

        let argument = match &segment.arguments {
            syn::PathArguments::AngleBracketed(arguments) => 'argument: {
                while let Some(arg) = arguments.args.iter().next() {
                    if let syn::GenericArgument::Type(ty) = arg {
                        break 'argument ty;
                    }
                }

                panic!("Expected a type argument for a vector type.");
            }
            t => panic!(
                "Expected angle bracketed arguments for a vector type. Got: {:?}",
                t
            ),
        };

        let ident = match argument {
            syn::Type::Path(path) => path.path.get_ident().unwrap().to_string(),
            t => panic!("Expected a path argument for a vector type. Got: {:?}", t),
        };

        quote! {
            {
                "name": stringify!(#name),
                "type": "list",
                "of": [#ident],
            }
        }
    }
}
