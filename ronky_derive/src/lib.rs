mod metadata;
mod parsers;

use parsers::{ParsedField, attributes::properties, parse_field};
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DataStruct, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

#[proc_macro]
pub fn export_stream(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => &fields.named,
        _ => {
            return quote_spanned!(input.span() => compile_error!("Only named structs are exportable for now"))
                .into();
        }
    };

    let metadata: proc_macro2::TokenStream = metadata::extract(&input.ident, &input.attrs).into();
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

    let ident_name = &input.ident.to_string();
    let mut properties = Vec::new();
    for field in fields.iter() {
        let field = parse_field(ident_name, field);
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
        use ronky::Serializable;
        let mut schema = ronky::PropertiesSchema::new();
        schema.set_metadata(#metadata);
        #attrs
        #(#properties)*
        schema
    }
    .into()
}

#[proc_macro_derive(Exported, attributes(arri))]
pub fn exported_derive(input: TokenStream) -> TokenStream {
    let export: proc_macro2::TokenStream = export_stream(input.clone()).into();

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();

    quote! {
        impl ronky::Exportable for #struct_name {
            fn export() -> ronky::PropertiesSchema {
                eprintln!("Exporting {}", stringify!(#struct_name));
                #export
            }
        }
    }
    .into()
}
