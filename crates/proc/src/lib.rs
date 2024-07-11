mod fmt;
mod formatter;

extern crate proc_macro;

use fmt::format_field;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse, DeriveInput};

/// This macro will generate a JSON representation of a struct with its fields.
#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();
    let struct_name = &input.ident.to_string();

    let data = match input.data {
        syn::Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match data.fields {
        syn::Fields::Named(fields) => fields.named,
        _ => panic!("Only named fields are supported for now"),
    };

    let fields = fields.iter().map(format_field);

    let expanded = quote! {
        serde_json::json!({
            "name": #struct_name,
            "fields": [
                #(#fields),*
            ]
        })
    };

    expanded.into()
}

/// Implement the Export trait for a struct. This does a compile time traversal of the struct its
/// AST properties.
#[proc_macro_derive(Export)]
pub fn export_struct(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();
    let struct_name = input.clone().ident;
    let result = export_stream(input.into_token_stream().into());
    let quotable_result: proc_macro2::TokenStream = result.into();

    let expanded = quote! {
        impl Exported for #struct_name {
            fn export() -> serde_json::Value {
                #quotable_result
            }
        }
    };

    expanded.into()
}
