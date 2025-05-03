use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use crate::{
    metadata,
    parsers::{ParsedField, attributes::properties, parse_field},
};

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
            // TODO: find out way to prevent the duplication here
            Ok(ParsedField::Required(field, stream, args)) => {
                let default_field_name = field.ident.as_ref().unwrap().to_string();
                let field_name = args
                    .into_iter()
                    .find(|a| a.rename.is_some())
                    .and_then(|a| a.rename)
                    .unwrap_or(default_field_name);

                // let field_name = args.and_then(|a| a.rename).unwrap_or(default_field_name);
                let stream: proc_macro2::TokenStream = stream.into();
                let field_metadata: Option<proc_macro2::TokenStream> =
                    metadata::extract_from_field(field).map(|ts| {
                        let ts: proc_macro2::TokenStream = ts.into();
                        quote! {
                            use ronky::Serializable;
                            ty.set_metadata(#ts);
                        }
                    });

                properties.push(quote! {
                    schema.set_property(#field_name, Box::new({
                        let mut ty = #stream;
                        #field_metadata;
                        ty
                    }));
                })
            }
            Ok(ParsedField::Optional(field, stream, args)) => {
                let default_field_name = field.ident.as_ref().unwrap().to_string();
                let field_name = args
                    .into_iter()
                    .find(|a| a.rename.is_some())
                    .and_then(|a| a.rename)
                    .unwrap_or(default_field_name);

                let stream: proc_macro2::TokenStream = stream.into();
                let field_metadata: Option<proc_macro2::TokenStream> =
                    metadata::extract_from_field(field).map(|ts| {
                        let ts: proc_macro2::TokenStream = ts.into();
                        quote! {
                            use ronky::Serializable;
                            ty.set_metadata(#ts);
                        }
                    });
                properties.push(quote! {
                    schema.set_optional_property(#field_name, Box::new({
                        let mut ty = #stream;
                        #field_metadata;
                        ty
                    }));
                })
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
