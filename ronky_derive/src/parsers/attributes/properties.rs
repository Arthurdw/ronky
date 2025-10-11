use super::goto_next;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
};
use proc_macro::TokenStream;
use syn::{
    Attribute, Ident, LitBool, LitStr,
    parse::{Parse, ParseStream},
    token,
};

use super::parse_arri_attrs;

/// Supported casing transformations for rename_all.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum CaseTransform {
    CamelCase,
    PascalCase,
    SnakeCase,
    ScreamingSnakeCase,
    KebabCase,
    ScreamingKebabCase,
}

impl CaseTransform {
    /// Parses a string into a CaseTransform variant.
    pub(crate) fn from_str(s: &str) -> Option<Self> {
        match s {
            "camelCase" => Some(Self::CamelCase),
            "PascalCase" => Some(Self::PascalCase),
            "snake_case" => Some(Self::SnakeCase),
            "SCREAMING_SNAKE_CASE" => Some(Self::ScreamingSnakeCase),
            "kebab-case" => Some(Self::KebabCase),
            "SCREAMING-KEBAB-CASE" => Some(Self::ScreamingKebabCase),
            _ => None,
        }
    }

    /// Applies the case transformation to a field name.
    pub(crate) fn transform(&self, name: &str) -> String {
        match self {
            Self::CamelCase => name.to_lower_camel_case(),
            Self::PascalCase => name.to_pascal_case(),
            Self::SnakeCase => name.to_snake_case(),
            Self::ScreamingSnakeCase => name.to_shouty_snake_case(),
            Self::KebabCase => name.to_kebab_case(),
            Self::ScreamingKebabCase => name.to_shouty_kebab_case(),
        }
    }
}

/// Represents parsed properties arguments.
///
/// This structure is used to store the parsed attributes for properties, such as the `strict` flag.
#[derive(Debug, Default)]
pub(crate) struct PropertiesArguments {
    /// Indicates whether the `strict` property is enabled.
    /// None means not set, Some(true) means enabled, Some(false) means explicitly disabled.
    pub(crate) strict: Option<bool>,
    /// Optional rename_all transformation for all fields.
    pub(crate) rename_all: Option<CaseTransform>,
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
                        args.strict = Some(value.value());
                    } else {
                        args.strict = Some(true);
                    }
                }
                "rename_all" => {
                    if input.peek(token::Eq) {
                        input.parse::<token::Eq>()?;
                        let value: LitStr = input.parse()?;
                        let transform_str = value.value();

                        match CaseTransform::from_str(&transform_str) {
                            Some(transform) => args.rename_all = Some(transform),
                            None => {
                                return Err(syn::Error::new(
                                    value.span(),
                                    format!(
                                        "Invalid rename_all value: '{}'. Supported values are: camelCase, PascalCase, snake_case, SCREAMING_SNAKE_CASE, kebab-case, SCREAMING-KEBAB-CASE",
                                        transform_str
                                    ),
                                ))?;
                            }
                        }
                    } else {
                        return Err(input.error("Expected '=' after 'rename_all'"))?;
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
pub(crate) fn extract(attrs: &[Attribute]) -> Result<Vec<PropertiesArguments>, TokenStream> {
    parse_arri_attrs(attrs)
}
