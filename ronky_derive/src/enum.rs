use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Variant, punctuated::Punctuated, token::Comma};

use crate::{
    metadata,
    parsers::attributes::enum_variants::{self, enum_transformation_to_tokens},
};

pub fn export_enum(input: &DeriveInput, variants: &Punctuated<Variant, Comma>) -> TokenStream {
    let metadata: proc_macro2::TokenStream = metadata::extract(&input.ident, &input.attrs).into();
    let attrs = match enum_variants::extract(&input.attrs) {
        Ok(Some(attrs)) => {
            let transform = attrs
                .transform
                .iter()
                .map(enum_transformation_to_tokens)
                .collect::<Vec<proc_macro2::TokenStream>>();

            Some(quote! {
                schema.set_transforms(&[#(#transform),*]);
            })
        }
        Ok(None) => None,
        Err(stream) => Some(stream.into()),
    };

    let mut exported = Vec::new();
    for variant in variants.iter() {
        let variant_name = variant.ident.to_string();
        // TODO: followup on request of having a list of metadata as variants, as we can't provide
        // any docs or deprecation notices to a variant :/
        // See current state here: https://discord.com/channels/1272569268869005322/1272569269342965874/1363165596048429137
        exported.push(quote! {
            schema.add_variant(#variant_name);
        });
    }

    quote! {
        let mut schema = ronky::EnumSchema::new();
        schema.set_metadata(#metadata);
        #attrs
        #(#exported)*

        schema
    }
    .into()
}
