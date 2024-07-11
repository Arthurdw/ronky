extern crate proc_macro;

use quote::{quote, ToTokens};
use syn::{parse, DeriveInput, Field};

fn format_field(field: &Field) -> proc_macro2::TokenStream {
    let name = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    quote! {
        {
            "name": stringify!(#name),
            "type": stringify!(#ty),
        }
    }
}

#[proc_macro]
pub fn export_stream(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse(input).unwrap();
    let struct_name = &input.ident;

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
            "types": [
                {
                    "name": stringify!(#struct_name),
                    "fields": [
                        #(#fields),*
                    ]
                }
            ]
        })
    };

    expanded.into()
}

#[proc_macro_derive(Export)]
pub fn export_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
