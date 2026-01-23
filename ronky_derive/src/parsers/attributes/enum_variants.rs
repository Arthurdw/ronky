use super::goto_next;
use arri_common::EnumTransformation;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Ident, LitStr, Token, bracketed,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use super::parse_arri_attrs;

/// Macro to match an `EnumTransformation` variant and generate corresponding tokens.
macro_rules! enum_transformation_match {
    ($transform:expr => $($variant:ident),*) => {
        match $transform {
            $(
                EnumTransformation::$variant => quote! { ronky::EnumTransformation::$variant },
            )*
        }
    };
}

/// Converts an `EnumTransformation` into a token stream representation.
///
/// # Arguments
///
/// * `transform` - A reference to the `EnumTransformation` to convert.
///
/// # Returns
///
/// A `proc_macro2::TokenStream` representing the transformation.
pub fn enum_transformation_to_tokens(transform: &EnumTransformation) -> proc_macro2::TokenStream {
    enum_transformation_match!(
        transform => Uppercase, Lowercase, Snakecase, Camelcase, Pascalcase,
                     Kebabcase, Screamingkebabcase, Screamingsnakecase
    )
}

/// Represents parsed attributes for enum variants.
#[derive(Debug, Default)]
pub(crate) struct EnumVariants {
    /// List of transformations to apply to the enum variants.
    pub(crate) transform: Vec<EnumTransformation>,
    /// Optional discriminator value for the enum variants.
    pub(crate) discriminator: Option<String>,
}

impl Parse for EnumVariants {
    /// Parses the input stream to extract `EnumVariants` attributes.
    ///
    /// # Arguments
    ///
    /// * `input` - The input parse stream.
    ///
    /// # Returns
    ///
    /// A `syn::Result` containing the parsed `EnumVariants` or an error.
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

/// Extracts `EnumVariants` attributes from a list of attributes.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to parse.
///
/// # Returns
///
/// A `Result` containing an optional `EnumVariants` or a `TokenStream` error.
pub(crate) fn extract(attrs: &[Attribute]) -> Result<Vec<EnumVariants>, TokenStream> {
    parse_arri_attrs(attrs)
}
