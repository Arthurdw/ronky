use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use crate::{
    metadata,
    parsers::{
        ParsedField,
        attributes::properties::{self, CaseTransform},
        parse_field,
    },
};

/// A macro to process a field and generate code for setting its properties in a schema.
///
/// # Parameters
/// - `$properties`: The collection to which the generated code will be pushed.
/// - `$field`: The field being processed.
/// - `$stream`: The type information or other data associated with the field.
/// - `$args`: Additional arguments or metadata for the field.
/// - `$set_property`: The method or function to set the property in the schema.
/// - `$rename_all`: Optional case transformation to apply to field names.
macro_rules! process_field {
    ($properties:ident => $field:expr, $stream:expr, $args:expr, $set_property:ident, $rename_all:expr) => {{
        // Extract the default field name from the field's identifier.
        let default_field_name = $field.ident.as_ref().unwrap().to_string();

        // Strip the r# prefix if present (for raw identifiers like r#type)
        let default_field_name = default_field_name
            .strip_prefix("r#")
            .unwrap_or(&default_field_name)
            .to_string();

        // Determine the field name, allowing for renaming via arguments.
        let field_name = $args
            .into_iter()
            .find(|a| a.rename.is_some())
            .and_then(|a| a.rename)
            .unwrap_or_else(|| {
                // Apply rename_all transformation if no explicit rename is provided
                match $rename_all {
                    Some(transform) => transform.transform(&default_field_name),
                    None => default_field_name,
                }
            });

        // Convert the stream into a TokenStream for further processing.
        let stream: proc_macro2::TokenStream = $stream.into();

        // Extract metadata from the field, if available, and generate code to set it.
        let field_metadata: Option<proc_macro2::TokenStream> = metadata::extract_from_field($field)
            .map(|ts| {
                let ts: proc_macro2::TokenStream = ts.into();
                quote! {
                    use ronky::Serializable;
                    ty.set_metadata(#ts);
                }
            });

        // Generate the code to set the field's property in the schema.
        $properties.push(quote! {
            schema.$set_property(#field_name, Box::new({
                let mut ty = #stream;
                #field_metadata;
                ty
            }));
        });
    }};
}

/// Exports the fields of a struct as a `TokenStream` for use in schema generation.
///
/// # Arguments
///
/// * `fields` - A reference to a `Punctuated` collection of `Field` objects representing the struct's fields.
/// * `rename_all` - Optional case transformation to apply to all field names.
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the struct's fields.
pub fn export_struct_fields(
    fields: &Punctuated<Field, Comma>,
    rename_all: &Option<CaseTransform>,
) -> TokenStream {
    let mut properties = Vec::new();
    for field in fields.iter() {
        match parse_field(field) {
            Ok(ParsedField::Required(field, stream, args)) => {
                process_field!(properties => field, stream, args, set_property, rename_all);
            }
            Ok(ParsedField::Optional(field, stream, args)) => {
                process_field!(properties => field, stream, args, set_optional_property, rename_all);
            }
            Err(stream) => return stream,
        }
    }

    quote! {
        let mut schema = ronky::PropertiesSchema::new();
        #(#properties)*
        schema
    }
    .into()
}

/// Exports a named struct as a `TokenStream` for use in schema generation.
///
/// # Arguments
///
/// * `input` - A reference to the `DeriveInput` representing the struct.
/// * `fields` - A reference to a `Punctuated` collection of `Field` objects representing the struct's fields.
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the named struct.
pub fn export_named_struct(input: &DeriveInput, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let metadata: proc_macro2::TokenStream = metadata::extract(&input.attrs).into();
    let (attrs, rename_all) = match properties::extract(&input.attrs) {
        Ok(attrs) => {
            if attrs.is_empty() {
                (None, None)
            } else {
                let strict = attrs.iter().find_map(|a| a.strict);
                let rename_all = attrs.iter().find_map(|a| a.rename_all.clone());

                let attrs_tokens = strict.map(|strict_value| {
                    quote! {
                        schema.set_strict(#strict_value);
                    }
                });

                (attrs_tokens, rename_all)
            }
        }
        Err(stream) => (Some(stream.into()), None),
    };

    let base_export: proc_macro2::TokenStream = export_struct_fields(fields, &rename_all).into();

    quote! {
        use ronky::Serializable;
        let mut schema = { #base_export };
        schema.set_metadata(#metadata);
        #attrs
        schema
    }
    .into()
}
