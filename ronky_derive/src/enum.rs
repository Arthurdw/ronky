use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{DeriveInput, Fields, Variant, punctuated::Punctuated, spanned::Spanned, token::Comma};

use crate::{
    metadata,
    named_struct::export_struct_fields,
    parsers::attributes::enum_variants::{self, enum_transformation_to_tokens},
};

pub fn export_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> TokenStream {
    let metadata: proc_macro2::TokenStream = metadata::extract(&input.ident, &input.attrs).into();

    let mut is_tagged_union = false;
    let mut exported = Vec::new();
    for (idx, variant) in variants.iter().enumerate() {
        let variant_name = variant.ident.to_string();

        if idx != 0
            && ((variant.fields.is_empty() && is_tagged_union)
                || (!variant.fields.is_empty() && !is_tagged_union))
        {
            return quote_spanned!(
                variant.span() =>
                compile_error!("Arri requires that Enums can only be all enum or all tagged union variants. This variant violates that rule.");
            ).into();
        }

        is_tagged_union = !variant.fields.is_empty();

        if is_tagged_union {
            match variant.fields {
                Fields::Named(ref fields) => {
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
                _ => {
                    return quote_spanned!(
                        variant.span() =>
                        compile_error!("Tagged union variants must have named fields.");
                    )
                    .into();
                }
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

    let attrs = match enum_variants::extract(&input.attrs) {
        Ok(Some(attrs)) => {
            let transform = attrs
                .transform
                .iter()
                .map(enum_transformation_to_tokens)
                .collect::<Vec<proc_macro2::TokenStream>>();

            let discriminator = match attrs.discriminator {
                Some(discriminator) if !is_tagged_union => {
                    return quote_spanned!(discriminator.span() =>
                        compile_error!("Discriminator can only be used with tagged unions.");
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
        Ok(None) => None,
        Err(stream) => Some(stream.into()),
    };

    let schema = if is_tagged_union {
        quote!(ronky::TaggedUnionSchema::new())
    } else {
        quote!(ronky::EnumSchema::new())
    };

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
