use super::goto_next;
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitStr,
    parse::{Parse, ParseStream},
    token,
};

use super::parse_arri_attrs;

/// Represents parsed attributes for struct fields.
#[derive(Debug, Default)]
pub(crate) struct FieldArguments {
    /// Optional rename value for the field.
    pub(crate) rename: Option<String>,
}

impl Parse for FieldArguments {
    /// Parses the input stream to extract `FieldArguments` attributes.
    ///
    /// # Arguments
    ///
    /// * `input` - The input parse stream.
    ///
    /// # Returns
    ///
    /// A `syn::Result` containing the parsed `FieldArguments` or an error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "rename" => {
                    if input.peek(token::Eq) {
                        input.parse::<token::Eq>()?;
                        let value: LitStr = input.parse()?;
                        let new_name = value.value();

                        if new_name.is_empty() {
                            return Err(syn::Error::new(value.span(), "A rename cannot be empty"))?;
                        } else if new_name.contains(' ') {
                            return Err(syn::Error::new(
                                value.span(),
                                "A rename cannot contain spaces",
                            ))?;
                        } else if new_name.starts_with(|c: char| c.is_numeric()) {
                            return Err(syn::Error::new(
                                value.span(),
                                "A rename cannot start with a number",
                            ))?;
                        } else if new_name.chars().any(|c| !c.is_ascii_alphanumeric()) {
                            return Err(syn::Error::new(
                                value.span(),
                                "A rename can only contain a-z, A-Z and 0-9",
                            ))?;
                        }
                        args.rename = Some(new_name);
                    } else {
                        return Err(input.error("Expected '=' after 'rename'"))?;
                    }
                }
                _ => {
                    // TODO: create a simple trait or smth that auto implements this and makes it a lot easier
                    // For unknown attributes, skip their values if present
                    if input.peek(token::Eq) {
                        input.parse::<token::Eq>()?;

                        // Ignore all other content
                        while !input.is_empty() && !input.peek(token::Comma) {
                            let _: proc_macro2::TokenTree = input.parse()?;
                        }
                    }

                    // Optionally, you could collect these for warnings
                    // warnings.push((key.span(), format!("Unknown attribute: {}", key_str)));
                }
            }

            goto_next(input)?;
        }

        Ok(args)
    }
}

/// Extracts `FieldArguments` attributes from a list of attributes.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to parse.
///
/// # Returns
///
/// A `Result` containing an optional `FieldArguments` or a `TokenStream` error.
pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<FieldArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
