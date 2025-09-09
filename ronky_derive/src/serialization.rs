use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_serialization(_input: &DeriveInput) -> TokenStream {
    // Don't generate any serialization code
    // The serialization will only be available when users explicitly add serde derives
    quote! {}
}
