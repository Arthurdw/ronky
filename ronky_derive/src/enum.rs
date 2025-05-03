use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Fields, Variant, punctuated::Punctuated, spanned::Spanned, token::Comma};

use crate::{
    metadata,
    named_struct::export_struct_fields,
    parsers::{
        ParsedField,
        attributes::{
            enum_variants::{self, enum_transformation_to_tokens},
            fields,
        },
        parse_field,
    },
};

/// Exports an enum as a `TokenStream` for use in schema generation.
///
/// # Arguments
///
/// * `input` - A reference to the `DeriveInput` representing the enum.
/// * `variants` - A reference to a `Punctuated` collection of `Variant` objects representing the enum's variants.
///
/// # Returns
///
/// Returns a `TokenStream` that defines the schema for the enum.
pub fn export_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> TokenStream {
    let metadata: proc_macro2::TokenStream = metadata::extract(&input.attrs).into();

    let mut is_tagged_union = false;
    let mut exported = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        // Ensure all variants are either tagged union or regular enum variants
        if idx != 0
            && ((variant.fields.is_empty() && is_tagged_union)
                || (!variant.fields.is_empty() && !is_tagged_union))
        {
            return quote_spanned!(
                variant.span() =>
                compile_error!("Arri requires that enums can only be all enum or all tagged union variants. This variant violates that rule.");
            )
            .into();
        }

        // Extract attributes for the variant
        let attrs = match fields::extract(&variant.attrs) {
            Ok(attrs) => attrs,
            Err(e) => return e,
        };

        let variant_name = attrs
            .iter()
            .rev()
            .find(|a| a.rename.is_some())
            .and_then(|a| a.rename.as_ref())
            .map(|s| s.to_string())
            .unwrap_or_else(|| variant.ident.to_string());

        is_tagged_union = !variant.fields.is_empty();

        if is_tagged_union {
            match variant.fields {
                Fields::Named(ref fields) => {
                    // Handle named fields in tagged union variants
                    let metadata: Option<proc_macro2::TokenStream> =
                        metadata::extract_attrs(&variant.attrs).map(|ts| {
                            let ts: proc_macro2::TokenStream = ts.into();
                            quote! {
                                use ronky::Serializable;
                                export.set_metadata(#ts);
                            }
                        });
                    let struct_export: proc_macro2::TokenStream =
                        export_struct_fields(&fields.named).into();

                    exported.push(quote! {
                        schema.add_mapping(#variant_name, Box::new({
                            let mut export = {#struct_export};
                            #metadata
                            export
                        }));
                    });
                }
                Fields::Unnamed(ref fields) => {
                    // Handle unnamed fields in tagged union variants
                    if fields.unnamed.len() != 1 {
                        return quote_spanned!(
                            variant.span() =>
                            compile_error!("Unamed enums variants must have exactly one field.");
                        )
                        .into();
                    }

                    let metadata: Option<proc_macro2::TokenStream> =
                        metadata::extract_attrs(&variant.attrs).map(|ts| {
                            let ts: proc_macro2::TokenStream = ts.into();
                            quote! {
                                use ronky::Serializable;
                                export.set_metadata(#ts);
                            }
                        });

                    let (field_stream, field_metadata) = match parse_field(
                        fields.unnamed.first().unwrap(),
                    ) {
                        Ok(ParsedField::Required(field, stream, ..)) => {
                            let stream: proc_macro2::TokenStream = stream.into();
                            let field_metadata: Option<proc_macro2::TokenStream> =
                                metadata::extract_from_field(field).map(|ts| {
                                    let ts: proc_macro2::TokenStream = ts.into();
                                    quote! {
                                        use ronky::Serializable;
                                        ty.set_metadata(#ts);
                                    }
                                });
                            (stream, field_metadata)
                        }
                        Ok(ParsedField::Optional(..)) => {
                            return quote_spanned!(
                                variant.span() =>
                                compile_error!("Optional fields are not supported in unnamed enums.")
                            )
                            .into();
                        }
                        Err(e) => return e,
                    };

                    exported.push(quote! {
                        schema.add_mapping(#variant_name, Box::new({
                            let mut export = ronky::PropertiesSchema::new();
                            #metadata
                            export.set_property("value", Box::new({
                                let mut ty = #field_stream;
                                #field_metadata
                                ty
                            }));
                            export
                        }));
                    });
                }
                _ => unreachable!(
                    "This will never be reached, as we already checked for empty fields"
                ),
            }
        } else {
            // TODO: followup on request of having a list of metadata as variants, as we can't provide
            // any docs or deprecation notices to a variant :/
            // See current state here: https://discord.com/channels/1272569268869005322/1272569269342965874/1363165596048429137
            exported.push(quote! {
                schema.add_variant(#variant_name);
            });
        }
    }

    // Extract attributes for the enum
    let attrs = match enum_variants::extract(&input.attrs) {
        Ok(attrs) => {
            let transform = attrs
                .iter()
                .flat_map(|attr| attr.transform.iter().map(enum_transformation_to_tokens))
                .collect::<Vec<proc_macro2::TokenStream>>();

            let found_discriminator = attrs.into_iter().find_map(|attr| attr.discriminator);

            let discriminator = match found_discriminator {
                Some(discriminator) if !is_tagged_union => {
                    return quote_spanned!(discriminator.span() =>
                        compile_error!("Discriminator can only be used with tagged enums.");
                    )
                    .into();
                }
                Some(discriminator) => Some(quote! {
                    schema.set_discriminator(#discriminator);
                }),
                None => None,
            };

            Some(quote! {
                schema.set_transforms(&[#(#transform),*]);
                #discriminator
            })
        }
        Err(stream) => Some(stream.into()),
    };

    // Determine the schema type based on whether it is a tagged union
    let schema = if is_tagged_union {
        quote!(ronky::TaggedUnionSchema::new())
    } else {
        quote!(ronky::EnumSchema::new())
    };

    // Generate the final schema
    quote! {
        use ronky::Serializable;
        let mut schema = #schema;
        schema.set_metadata(#metadata);
        #attrs
        #(#exported)*

        schema
    }
    .into()
}
