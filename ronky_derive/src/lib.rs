//! # Ronky Derive Macro Library
//!
//! This crate provides procedural macros for the `ronky` library, enabling seamless implementation
//! of traits and boilerplate code generation for working with Arri schemas.
//!
//! The primary macro provided by this crate is `Exported`, which implements the `Exportable` trait
//! for structs and enums. This trait facilitates the conversion of types into representations
//! defined by the `arri_repr` crate, making it easier to work with Arri schema elements.
//!
//! ## Features
//! - Derive macros for implementing the `Exportable` trait.
//! - Simplifies the process of converting types to `arri_repr` representations.
//! - Designed to integrate tightly with the `ronky` crate.
//!
//! ## Usage
//! Use the `Exported` macro to automatically implement the `Exportable` trait for your types:
//!
//! ```rust
//! use ronky_derive::Exported;
//!
//! #[derive(Exported)]
//! struct MySchema {
//!     name: String,
//!     value: i32,
//! }
//! ```
//!
//! If you're looking for the crate that provides the core schema manipulation utilities, you are
//! probably looking for the [ronky](https://docs.rs/ronky) crate.

mod r#enum;
mod metadata;
mod named_struct;
mod parsers;

use r#enum::export_enum;
use named_struct::export_named_struct;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

/// A procedural macro to export a struct or enum.
///
/// This macro processes the input token stream and determines whether the input
/// is a named struct or an enum. Based on the input type, it delegates the export
/// logic to the appropriate module.
///
/// # Errors
/// - Emits a compile-time error if the input is neither a named struct nor an enum.
#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(DataStruct { fields: Fields::Named(ref fields), .. }) => export_named_struct(&input, &fields.named),
        Data::Enum(DataEnum { ref variants, .. }) => export_enum(&input, variants),
        _ => quote_spanned!(input.span() => compile_error!("Only named structs or enums are exportable for now")).into()
    }
}

/// A procedural macro to derive the `Exported` trait for structs and enums.
///
/// This macro generates an implementation of the `ronky::Exportable` trait for the
/// annotated struct or enum. It also provides a specialized implementation of the
/// `get_type_name` method for generic types.
///
/// # Attributes
/// - `#[arri]`: Custom attributes supported by this macro.
///
/// # Example
/// ```ignore
/// #[derive(Exported)]
/// struct MyStruct {
///     field: String,
/// }
/// ```
#[proc_macro_derive(Exported, attributes(arri))]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let export: proc_macro2::TokenStream = export_stream(input.clone()).into();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract the type parameters from generics
    let type_params = generics
        .type_params()
        .map(|param| param.ident.clone())
        .collect::<Vec<_>>();

    // Only generate the specialized get_type_name implementation if there are generic arguments
    let get_type_name_impl = if !type_params.is_empty() {
        quote! {
            /// Returns the type name of the struct or enum, including generic parameters.
            fn get_type_name() -> String {
                format!(
                    "::ronky::--virtual--::external::{}",
                    vec![stringify!(#struct_name).to_string(), #(#type_params::get_type_name()),*].join("")
                )
            }
        }
    } else {
        // For non-generic types, use the default implementation
        quote! {}
    };

    quote! {
        impl #impl_generics ronky::Exportable for #struct_name #ty_generics #where_clause {
            /// Exports the struct or enum as a serializable representation.
            fn export_internal() -> impl ronky::Serializable {
                #export
            }
            #get_type_name_impl
        }
    }
    .into()
}
