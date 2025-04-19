use super::goto_next;
use arri_repr::EnumTransformation;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, LitStr, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use super::parse_arri_attrs;

pub fn enum_transformation_to_tokens(transform: &EnumTransformation) -> proc_macro2::TokenStream {
    // TODO: there should be a cleaner way to do this (probb a macro rules)
    match transform {
        EnumTransformation::Uppercase => quote! { ronky::EnumTransformation::Uppercase },
        EnumTransformation::Lowercase => quote! { ronky::EnumTransformation::Lowercase },
        EnumTransformation::Snakecase => quote! { ronky::EnumTransformation::Snakecase },
        EnumTransformation::Camelcase => quote! { ronky::EnumTransformation::Camelcase },
        EnumTransformation::Pascalcase => quote! { ronky::EnumTransformation::Pascalcase },
    }
}

#[derive(Debug, Default)]
pub(crate) struct EnumVariants {
    pub(crate) transform: Vec<EnumTransformation>,
    pub(crate) discriminator: Option<String>,
}

impl Parse for EnumVariants {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "transform" => {
                    if !input.peek(syn::token::Eq) {
                        return Err(input.error("Expected '=' after 'transform'"));
                    }

                    input.parse::<syn::token::Eq>()?;

                    let transforms = if input.peek(syn::token::Bracket) {
                        // Parse a list of transformations inside brackets
                        let content;
                        let _bracket_token = bracketed!(content in input);

                        // Parse comma-separated list of string literals
                        let transform_list =
                            Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;

                        // Convert to Vec<String>
                        transform_list
                            .into_iter()
                            .map(|lit| lit.value())
                            .collect::<Vec<String>>()
                    } else {
                        // Parse a single transformation as a string literal
                        let transform_lit = input.parse::<LitStr>()?;
                        vec![transform_lit.value()]
                    };

                    let transforms = transforms
                        .into_iter()
                        .map(|s| EnumTransformation::try_from(s).map_err(|e| input.error(e)))
                        .collect::<Result<Vec<_>, _>>()?;

                    args.transform.extend(transforms);
                }
                "discriminator" => {
                    if !input.peek(syn::token::Eq) {
                        return Err(input.error("Expected '=' after 'discriminator'"));
                    }

                    input.parse::<syn::token::Eq>()?;

                    let discriminator_lit = input.parse::<LitStr>()?;
                    args.discriminator = Some(discriminator_lit.value());
                }
                _ => Err(input.error(format!("Unknown property: {}", key_str)))?,
            }

            goto_next(input)?;
        }

        Ok(args)
    }
}

pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<EnumVariants>, TokenStream> {
    parse_arri_attrs(attrs)
}
