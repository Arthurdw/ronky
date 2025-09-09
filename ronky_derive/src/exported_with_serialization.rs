use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Creates a new proc macro attribute that automatically adds sonic-rs derives
pub fn add_serialization_derives(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args; // Ignore arguments for now
    let mut input = parse_macro_input!(input as DeriveInput);

    // Add the sonic-rs derive attributes
    let serialize_attr = syn::parse_quote!(#[derive(sonic_rs::Serialize, sonic_rs::Deserialize)]);
    input.attrs.push(serialize_attr);

    quote! {
        #input
    }
    .into()
}
