use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, punctuated::Punctuated, token::Comma};

use crate::{
    metadata,
    parsers::{ParsedField, attributes::properties, parse_field},
};

pub fn export_struct_fields(fields: &Punctuated<Field, Comma>) -> TokenStream {
    let mut properties = Vec::new();
    for field in fields.iter() {
        let field = parse_field(field);
        match field {
            // TODO: find out way to prevent the duplication here
            Ok(ParsedField::Required(field, stream)) => {
                let field_name = field.ident.as_ref().unwrap().to_string();
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
            Ok(ParsedField::Optional(field, stream)) => {
                let field_name = field.ident.as_ref().unwrap().to_string();
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

pub fn export_named_struct(input: &DeriveInput, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let metadata: proc_macro2::TokenStream = metadata::extract(&input.attrs).into();
    let attrs = match properties::extract(&input.attrs) {
        Ok(Some(attrs)) => {
            let strict = attrs.strict;

            Some(quote! {
                schema.set_strict(#strict);
            })
        }
        Ok(None) => None,
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
