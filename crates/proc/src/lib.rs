extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{parse, DeriveInput};

#[proc_macro_derive(Export)]
pub fn export_struct(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse(input).unwrap();

    let struct_name = &input.ident;

    let expanded = quote! {
        impl Exported for #struct_name {
            fn export() -> serde_json::Value {
                serde_json::json!({
                    "name": stringify!(#struct_name),
                    // TODO: implement the rest
                })
            }
        }
    };

    expanded.into()
}
