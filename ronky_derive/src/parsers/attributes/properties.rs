use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
};

use super::{parse_flag, parse_required_string};

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

define_arri_attrs! {
    /// Represents parsed properties arguments.
    pub(crate) struct PropertiesArguments {
        /// Indicates whether the `strict` property is enabled.
        pub(crate) strict: Option<bool>,
        /// Optional rename_all transformation for all fields.
        pub(crate) rename_all: Option<CaseTransform>,
    }

    parse(args, input) {
        "strict" => {
            args.strict = Some(parse_flag(input)?);
        }
        "rename_all" => {
            let value = parse_required_string(input, "rename_all")?;
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
                    ));
                }
            }
        }
    }
}
