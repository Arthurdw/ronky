use super::goto_next;
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitBool,
    parse::{Parse, ParseStream},
    token,
};

use super::parse_arri_attrs;

/// Represents parsed properties arguments.
///
/// This structure is used to store the parsed attributes for properties, such as the `strict` flag.
#[derive(Debug, Default)]
pub(crate) struct PropertiesArguments {
    /// Indicates whether the `strict` property is enabled.
    pub(crate) strict: bool,
}

impl Parse for PropertiesArguments {
    /// Parses the input stream to extract `PropertiesArguments` attributes.
    ///
    /// # Arguments
    ///
    /// * `input` - The input parse stream.
    ///
    /// # Returns
    ///
    /// A `syn::Result` containing the parsed `PropertiesArguments` or an error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "strict" => {
                    if input.peek(token::Eq) {
                        input.parse::<token::Eq>()?;
                        let value: LitBool = input.parse()?;
                        args.strict = value.value();
                    } else {
                        args.strict = true;
                    }
                }
                _ => Err(input.error(format!("Unknown property: {}", key_str)))?,
            }

            goto_next(input)?;
        }

        Ok(args)
    }
}

/// Extracts `PropertiesArguments` attributes from a list of attributes.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to parse.
///
/// # Returns
///
/// A `Result` containing an optional `PropertiesArguments` or a `TokenStream` error.
pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<PropertiesArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
