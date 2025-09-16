#![allow(clippy::multiple_crate_versions)]

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
//! ```rust,ignore
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
#[cfg(feature = "serialization")]
mod serialization;

use r#enum::export_enum;
use heck::ToLowerCamelCase;
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
/// NOTE: You must also derive `sonic_rs::Serialize` and `sonic_rs::Deserialize` for serialization support.
///
/// # Attributes
/// - `#[arri]`: Custom attributes supported by this macro.
///
/// # Example
/// ```ignore
/// #[derive(Exported, sonic_rs::Serialize, sonic_rs::Deserialize)]
/// struct MyStruct {
///     field: String,
/// }
/// ```
#[proc_macro_derive(Exported, attributes(arri))]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let export: proc_macro2::TokenStream = export_stream(input.clone()).into();
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    let generics = &input.generics;
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

    // Only generate serialization if the feature is enabled in the derive crate
    let serialization_impl = {
        #[cfg(feature = "serialization")]
        {
            crate::serialization::generate_serialization(&input)
        }
        #[cfg(not(feature = "serialization"))]
        {
            quote! {}
        }
    };

    quote! {
        impl #impl_generics ronky::Exportable for #struct_name #ty_generics #where_clause {
            /// Exports the struct or enum as a serializable representation.
            fn export_internal() -> impl ronky::Serializable {
                #export
            }
            #get_type_name_impl
        }

        #serialization_impl
    }
    .into()
}

/// A procedural macro to derive the `Serializable` trait for structs.
///
/// This macro generates an implementation of the `arri_repr::Serializable` trait for the
/// annotated struct. It automatically generates the `serialize()` method based on struct fields,
/// and optionally implements `set_metadata()` and `set_nullable()` if the corresponding fields exist.
///
/// # Field name transformations
/// - `is_deprecated` becomes `"isDeprecated"`
/// - `deprecated_since` becomes `"deprecatedSince"`
/// - `deprecated_message` becomes `"deprecatedNote"`
/// - Other `snake_case` fields are converted to `camelCase`
///
/// # Special field detection
/// - If a field named `metadata` of type `Option<MetadataSchema>` exists, `set_metadata()` will be implemented
/// - If a field named `nullable` or `is_nullable` of type `Option<bool>` exists, `set_nullable()` will be implemented
/// - Missing fields will generate warnings unless disabled with `#[arri_disable(metadata, nullable)]`
///
/// Raw identifiers like `r#type`, `r#ref`, and `r#enum` are supported and serialized without the `r#` prefix.
///
/// # Example
/// ```ignore
/// #[derive(Serializable)]
/// struct MySchema {
///     pub id: Option<String>,
///     pub description: Option<String>,
///     pub is_deprecated: Option<bool>,
///     pub metadata: Option<MetadataSchema>,  // Enables set_metadata()
///     pub nullable: Option<bool>,            // Enables set_nullable()
/// }
/// ```
#[proc_macro_derive(Serializable, attributes(arri_disable))]
pub fn serializable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => generate_serializable_impl(&input, &fields.named),
        _ => quote_spanned!(input.span() =>
            compile_error!("Serializable can only be derived for structs with named fields")
        )
        .into(),
    }
}

fn generate_serializable_impl(
    input: &DeriveInput,
    fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> TokenStream {
    let struct_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Determine target crate path robustly
    let crate_path = {
        use proc_macro_crate::{FoundCrate, crate_name};
        match crate_name("ronky") {
            Ok(FoundCrate::Itself) => quote! { crate },
            Ok(FoundCrate::Name(name)) => {
                let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
                quote! { ::#ident }
            }
            Err(_) => {
                // If ronky is not found, try arri_repr
                match crate_name("arri_repr") {
                    Ok(FoundCrate::Itself) => quote! { crate },
                    Ok(FoundCrate::Name(name)) => {
                        let ident = proc_macro2::Ident::new(&name, proc_macro2::Span::call_site());
                        quote! { ::#ident }
                    }
                    Err(_) => quote! { ::ronky }, // fallback
                }
            }
        }
    };

    // Check for disabled warnings
    let disabled_warnings = get_disabled_warnings(&input.attrs);

    // Generate serialize method
    let serialize_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let mut field_name_str = field_name.to_string();
        // Strip r# prefix from raw identifiers
        field_name_str = field_name_str
            .strip_prefix("r#")
            .unwrap_or(&field_name_str)
            .to_string();
        let json_key = transform_field_name(&field_name_str);

        quote! {
            .set(#json_key, &self.#field_name)
        }
    });

    // Check for metadata and nullable fields
    let has_metadata = fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|name| name == "metadata"));

    let nullable_field = fields
        .iter()
        .find(|f| {
            f.ident
                .as_ref()
                .is_some_and(|name| name == "nullable" || name == "is_nullable")
        })
        .and_then(|f| f.ident.as_ref());
    let has_nullable = nullable_field.is_some();

    // Generate warnings for missing fields
    let mut warnings = Vec::new();
    if !has_metadata && !disabled_warnings.iter().any(|s| s == "metadata") {
        warnings.push(quote! {
            const _: () = {
                #[deprecated(note = "Consider adding a 'metadata: Option<#crate_path::MetadataSchema>' field for better schema support, or disable this warning with #[arri_disable(metadata)]")]
                const _METADATA_FIELD_MISSING: () = ();
                _METADATA_FIELD_MISSING // trigger deprecation
            };
        });
    }

    if !has_nullable && !disabled_warnings.iter().any(|s| s == "nullable") {
        warnings.push(quote! {
            const _: () = {
                #[deprecated(note = "Consider adding a 'nullable: Option<bool>' or 'is_nullable: Option<bool>' field for nullability support, or disable this warning with #[arri_disable(nullable)]")]
                const _NULLABLE_FIELD_MISSING: () = ();
                _NULLABLE_FIELD_MISSING // trigger deprecation
            };
        });
    }

    // Generate set_metadata implementation if metadata field exists
    let set_metadata_impl = if has_metadata {
        quote! {
            fn set_metadata(&mut self, metadata: #crate_path::MetadataSchema) {
                self.metadata = Some(if let Some(current) = &self.metadata {
                    current.clone() | metadata
                } else {
                    metadata
                });
            }
        }
    } else {
        quote! {}
    };

    // Generate set_nullable implementation if nullable field exists
    let set_nullable_impl = if let Some(field_name) = nullable_field {
        quote! {
            fn set_nullable(&mut self, nullable: bool) {
                self.#field_name = Some(nullable);
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #(#warnings)*

        impl #impl_generics #crate_path::Serializable for #struct_name #ty_generics #where_clause {
            fn serialize(&self) -> Option<String> {
                #crate_path::Serializer::builder()
                    #(#serialize_calls)*
                    .build()
                    .into()
            }

            #set_metadata_impl
            #set_nullable_impl
        }
    }
    .into()
}

fn transform_field_name(field_name: &str) -> String {
    match field_name {
        "type" => "type".to_string(),
        "enum" => "enum".to_string(),
        "ref" => "ref".to_string(),
        "is_deprecated" => "isDeprecated".to_string(),
        "deprecated_since" => "deprecatedSince".to_string(),
        "deprecated_message" => "deprecatedNote".to_string(),
        "nullable" => "isNullable".to_string(),
        "is_nullable" => "isNullable".to_string(),
        "is_strict" => "isStrict".to_string(),
        "optional_properties" => "optionalProperties".to_string(),
        _ => {
            // Preserve all leading underscores; camelCase the remainder
            let underscores = field_name.chars().take_while(|&c| c == '_').count();
            if underscores > 0 {
                let rest = &field_name[underscores..];
                format!("{}{}", "_".repeat(underscores), rest.to_lower_camel_case())
            } else {
                field_name.to_lower_camel_case()
            }
        }
    }
}

fn get_disabled_warnings(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut disabled = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("arri_disable") {
            // Use syn's parsing utilities for more robust parsing
            if let Ok(args) = attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated,
            ) {
                for ident in args {
                    disabled.push(ident.to_string().to_lowercase().trim().to_string());
                }
            }
        }
    }

    disabled
}
