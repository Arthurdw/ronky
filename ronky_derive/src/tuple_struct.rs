use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Field, punctuated::Punctuated, spanned::Spanned, token::Comma};

/// Exports a tuple struct (struct with unnamed fields) as a `TokenStream` for use in schema generation.
///
/// # Arguments
///
/// * `input` - A reference to the `DeriveInput` representing the struct.
/// * `fields` - A reference to a `Punctuated` collection of `Field` objects representing the struct's fields.
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the tuple struct.
///
/// # Errors
///
/// - Returns a compile error if the tuple struct has no fields (unit-like tuple struct).
/// - Returns a compile error with a reference to the Arri tuple discussion if it has multiple fields.
pub fn export_tuple_struct(input: &DeriveInput, fields: &Punctuated<Field, Comma>) -> TokenStream {
    // Handle empty tuple structs (unit-like)
    if fields.is_empty() {
        return quote_spanned!(input.span() =>
            compile_error!("Empty tuple structs are not exportable")
        )
        .into();
    }

    // Handle multi-field tuple structs - reference the Arri discussion
    if fields.len() > 1 {
        return quote_spanned!(input.span() =>
            compile_error!("Multi-field tuple structs are not yet supported. See the Arri tuple discussion: https://github.com/modiimedia/arri/issues/163")
        )
        .into();
    }

    let field = fields.first().unwrap();
    let ty = &field.ty;

    // Generate the export code that delegates to the inner type's export_internal.
    // The wrapper is transparent - we export as if it were the inner type directly,
    // without adding the wrapper's metadata. This follows the discussion that
    // "the client doesn't really benefit from actually knowing that the value was wrapped in a type."
    quote! {
        <#ty as ronky::Exportable>::export_internal()
    }
    .into()
}
