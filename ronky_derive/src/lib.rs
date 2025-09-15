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
/// - If a field named `nullable` of type `Option<bool>` exists, `set_nullable()` will be implemented
/// - Missing fields will generate warnings unless disabled with `#[arri_disable(metadata, nullable)]`
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

    // Check for disabled warnings
    let disabled_warnings = get_disabled_warnings(&input.attrs);

    // Generate serialize method
    let serialize_calls = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let json_key = transform_field_name(&field_name_str);

        quote! {
            .set(#json_key, &self.#field_name)
        }
    });

    // Check for metadata and nullable fields
    let has_metadata = fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|name| name == "metadata"));

    let has_nullable = fields
        .iter()
        .any(|f| f.ident.as_ref().is_some_and(|name| name == "nullable"));

    // Generate warnings for missing fields
    let mut warnings = Vec::new();
    if !has_metadata && !disabled_warnings.contains(&"metadata".to_string()) {
        warnings.push(quote! {
            #[deprecated(note = "Consider adding a 'metadata: Option<MetadataSchema>' field for better schema support, or disable this warning with #[arri_disable(metadata)]")]
            const _METADATA_FIELD_MISSING: () = ();
        });
    }

    if !has_nullable && !disabled_warnings.contains(&"nullable".to_string()) {
        warnings.push(quote! {
            #[deprecated(note = "Consider adding a 'nullable: Option<bool>' field for nullability support, or disable this warning with #[arri_disable(nullable)]")]
            const _NULLABLE_FIELD_MISSING: () = ();
        });
    }

    // Generate set_metadata implementation if metadata field exists
    let set_metadata_impl = if has_metadata {
        quote! {
            fn set_metadata(&mut self, metadata: ::ronky::MetadataSchema) {
                self.metadata = Some(metadata);
            }
        }
    } else {
        quote! {}
    };

    // Generate set_nullable implementation if nullable field exists
    let set_nullable_impl = if has_nullable {
        quote! {
            fn set_nullable(&mut self, nullable: bool) {
                self.nullable = Some(nullable);
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #(#warnings)*

        impl #impl_generics ::ronky::Serializable for #struct_name #ty_generics #where_clause {
            fn serialize(&self) -> Option<String> {
                ::ronky::Serializer::builder()
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
        "is_deprecated" => "isDeprecated".to_string(),
        "deprecated_since" => "deprecatedSince".to_string(),
        "deprecated_message" => "deprecatedNote".to_string(),
        _ => snake_to_camel_case(field_name),
    }
}

fn snake_to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}

fn get_disabled_warnings(attrs: &[syn::Attribute]) -> Vec<String> {
    let mut disabled = Vec::new();

    for attr in attrs {
        if attr.path().is_ident("arri_disable") {
            // Parse the parenthesized content manually
            if let Ok(tokens) = attr.parse_args::<proc_macro2::TokenStream>() {
                // Convert tokens to string and split by commas
                let tokens_str = tokens.to_string();
                for item in tokens_str.split(',') {
                    let trimmed = item.trim();
                    if !trimmed.is_empty() {
                        disabled.push(trimmed.to_string());
                    }
                }
            }
        }
    }

    disabled
}
