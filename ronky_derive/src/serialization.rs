use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generates serialization implementation for the Exported derive macro.
///
/// This function intentionally returns empty tokens. The serialization feature
/// relies on blanket trait implementations in the main ronky crate that work
/// with any type implementing serde's Serialize/Deserialize traits.
///
/// Users must explicitly add `#[derive(serde::Serialize, serde::Deserialize)]`
/// to their types to enable serialization. This design choice ensures that:
/// - Serialization is opt-in per type
/// - We don't force serde derives on all Exported types
/// - Users have full control over serde attributes and configuration
pub fn generate_serialization(_input: &DeriveInput) -> TokenStream {
    quote! {}
}
