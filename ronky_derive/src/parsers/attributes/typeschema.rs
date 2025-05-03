use super::goto_next;
use super::parse_arri_attrs;
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitBool,
    parse::{Parse, ParseStream},
    token,
};

/// Represents parsed type schema arguments.
///
/// This structure is used to store the parsed attributes for type schemas, such as the `nullable` flag.
#[derive(Debug, Default)]
pub(crate) struct TypeSchemaArguments {
    /// Indicates whether the type is nullable.
    pub(crate) is_nullable: Option<bool>,
}

impl Parse for TypeSchemaArguments {
    /// Parses the input stream to extract `TypeSchemaArguments` attributes.
    ///
    /// # Arguments
    ///
    /// * `input` - The input parse stream.
    ///
    /// # Returns
    ///
    /// A `syn::Result` containing the parsed `TypeSchemaArguments` or an error.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let key_str = key.to_string();

            match key_str.as_str() {
                "nullable" => {
                    if input.peek(token::Eq) {
                        input.parse::<token::Eq>()?;
                        let value: LitBool = input.parse()?;
                        args.is_nullable = Some(value.value());
                    } else {
                        args.is_nullable = Some(true);
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

/// Extracts `TypeSchemaArguments` attributes from a list of attributes.
///
/// # Arguments
///
/// * `attrs` - A slice of `Attribute` objects to parse.
///
/// # Returns
///
/// A `Result` containing an optional `TypeSchemaArguments` or a `TokenStream` error.
pub(crate) fn extract(attrs: &[Attribute]) -> Result<Option<TypeSchemaArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
