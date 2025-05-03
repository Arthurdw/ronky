use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use crate::{
    metadata,
    parsers::{ParsedField, attributes::properties, parse_field},
};

/// A macro to process a field and generate code for setting its properties in a schema.
///
/// # Parameters
/// - `$properties`: The collection to which the generated code will be pushed.
/// - `$field`: The field being processed.
/// - `$stream`: The type information or other data associated with the field.
/// - `$args`: Additional arguments or metadata for the field.
/// - `$set_property`: The method or function to set the property in the schema.
macro_rules! process_field {
    ($properties:ident => $field:expr, $stream:expr, $args:expr, $set_property:ident) => {{
        // Extract the default field name from the field's identifier.
        let default_field_name = $field.ident.as_ref().unwrap().to_string();

        // Determine the field name, allowing for renaming via arguments.
        let field_name = $args
            .into_iter()
            .find(|a| a.rename.is_some())
            .and_then(|a| a.rename)
            .unwrap_or(default_field_name);

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
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the struct's fields.
pub fn export_struct_fields(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut properties = Vec::new();
    for field in fields.iter() {
        match parse_field(field) {
            Ok(ParsedField::Required(field, stream, args)) => {
                process_field!(properties => field, stream, args, set_property);
            }
            Ok(ParsedField::Optional(field, stream, args)) => {
                process_field!(properties => field, stream, args, set_optional_property);
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
    let attrs = match properties::extract(&input.attrs) {
        Ok(attrs) => {
            if attrs.is_empty() {
                None
            } else {
                let strict = attrs.iter().any(|a| a.strict);

                Some(quote! {
                    schema.set_strict(#strict);
                })
            }
        }
        Err(stream) => Some(stream.into()),
    };

    let base_export: proc_macro2::TokenStream = export_struct_fields(fields).into();

    quote! {
        use ronky::Serializable;
        let mut schema = { #base_export };
        schema.set_metadata(#metadata);
        #attrs
        schema
    }
    .into()
}
