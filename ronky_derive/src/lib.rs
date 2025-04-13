mod metadata;
mod parsers;

use parsers::{ParsedField, parse_field};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, parse_macro_input, spanned::Spanned};

#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => {
            return quote_spanned!(input.span() => compile_error!("Only structs are supported"))
                .into();
        }
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => {
            return quote_spanned!(input.span() => compile_error!("Only named structs are exportable for now"))
                .into();
        }
    };
    let metadata: proc_macro2::TokenStream = metadata::extract(&input).into();

    let properties = fields
        .iter()
        .map(parse_field)
        .filter_map(|field| match field {
            ParsedField::Required(field, stream) => {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let stream: proc_macro2::TokenStream = stream.into();
                Some(quote! {
                    schema.set_property(#field_name, Box::new(#stream));
                })
            }
            ParsedField::Optional(field, stream) => {
                let field_name = field.ident.as_ref().unwrap().to_string();
                let stream: proc_macro2::TokenStream = stream.into();
                Some(quote! {
                    schema.set_optional_property(#field_name, Box::new(#stream));
                })
            }
        });

    quote! {
        let mut schema = ronky::PropertiesSchema::new();
        schema.set_metadata(#metadata);
        #(#properties)*
        schema
    }
    .into()
}

#[proc_macro_derive(Exported)]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let quotable_result: proc_macro2::TokenStream = export_stream(input.clone()).into();

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    quote! {
        impl ronky::Exportable for #struct_name {
            fn export() -> ronky::PropertiesSchema {
                #quotable_result
            }
        }
    }
    .into()
}
