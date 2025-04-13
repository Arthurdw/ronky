mod metadata;
mod parsers;

use parsers::{ParsedField, parse_field};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let data = match input.data {
        syn::Data::Struct(ref data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match &data.fields {
        syn::Fields::Named(fields) => &fields.named,
        _ => panic!("Only named fields are supported for now"),
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
